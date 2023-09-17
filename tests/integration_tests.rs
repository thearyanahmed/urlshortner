#[cfg(test)]
mod integration_tests {
    use std::sync::{Arc, Mutex};
    use actix_web::http;
    use serde_json::json;
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use urlshortner::url_shortener::{configuration::Settings, redis::RedisStore, postgres::PostgresStore, UrlShortenerService, HttpServer};

    struct TestApp {
        app_url: String
    }

    use rand::Rng;

    fn generate_random_string() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();

        let random_string: String = (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..(52 as usize));
                CHARSET[idx] as char
            })
            .collect();

        random_string
    }


    async fn spawn_app() -> TestApp {
        let db_connection = "postgres://postgres:password@localhost:54321";
        let db_name = generate_random_string();

        configure_database(db_connection, db_name.as_str()).await;

        let db_connection_str = format!("{}/{}",db_connection,db_name);

        let config = Settings {
            base_url: "localhost".to_string(),
            port: 8080,
            url_prefix: "tier.app".to_string(),
            key_size: 7,
            cache_connection_url: "redis://localhost:6379/".to_string(),
            database_connection_url: db_connection_str.to_string(),
        };

        let config = Arc::new(config); // Wrap config in Arc

        let cache = RedisStore::new(&config.cache_connection_url);
        let db = PostgresStore::new(&config.database_connection_url).expect("failed to connect to db");

        let svc = UrlShortenerService::new(db, cache, &config.clone());
        let svc: Arc<Mutex<UrlShortenerService>> = Arc::new(Mutex::new(svc));

        let svc_clone = Arc::clone(&svc);
        let config_clone = Arc::clone(&config);

        let _ = tokio::spawn(async move {
            HttpServer::listen_and_serve(&config_clone, svc_clone).await.unwrap();
        });

        let app_url = format!("http://{}:{}",config.base_url,config.port);

        TestApp {
            app_url
        }
    }

    // Configures the database. Creates a connection pool and runs migration.
    pub async fn configure_database(connection_str: &str, db: &str) {
        let mut connection = PgConnection::connect(connection_str)
            .await.unwrap();

        // create a database
        connection.execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                db
            ).as_str()
        )
            .await.unwrap();

        let connection_string_with_db = format!("{}/{}",connection_str.to_string(),db);

        let connection_pool = PgPool::connect(&*connection_string_with_db).await.unwrap();

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await.unwrap()
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = spawn_app().await;

        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/health-check",app.app_url))
            .header("content-type","application/json")
            .send()
            .await
            .expect("failed to execute test");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_shorten_url() {
        let app = spawn_app().await;

        let client = reqwest::Client::new();

        let request_body = json!({ "url" : "https://google.com" }).to_string();

        let response = client
            .post(format!("{}/shorten",app.app_url))
            .header("content-type","application/json")
            .body(request_body)
            .send()
            .await
            .expect("failed to shorten url");

        assert!(response.status().is_success());

        let body = response.text().await.expect("Failed to read response body");
        let json_response: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

        assert!(json_response.is_object());
        assert!(json_response.get("short_url").is_some());
    }


    #[tokio::test]
    async fn test_visit() {
        let app = spawn_app().await;

        let client = reqwest::Client::new();

        let request_body = json!({ "url" : "https://apple.com" }).to_string();

        let response = client
            .post(format!("{}/shorten", app.app_url))
            .header("content-type", "application/json")
            .body(request_body)
            .send()
            .await
            .expect("failed to shorten url");

        assert!(response.status().is_success());

        let body = response.text().await.expect("Failed to read response body");
        let json_response: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");

        let short_url = json_response["short_url"].as_str().expect("short_url not found").to_string();

        let url_key = short_url.splitn(2, '/').nth(1).unwrap_or_default().to_string();

        assert_ne!(url_key, "");

        let response = client
            .get(format!("{}/visit/{}", app.app_url, url_key))
            .header("content-type", "application/json")
            .send()
            .await
            .expect("failed to shorten url");

        assert_eq!(response.status(), http::StatusCode::OK)
    }
}

