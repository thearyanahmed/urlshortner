-- Create the table to track visits
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    url_id INTEGER REFERENCES urls(id),
    visited_at TIMESTAMPTZ DEFAULT NOW()
);
