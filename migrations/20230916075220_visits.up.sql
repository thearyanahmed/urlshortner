-- Create the table to track visits
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    key varchar(10) REFERENCES urls(key),
    visited_at TIMESTAMPTZ DEFAULT NOW()
);
