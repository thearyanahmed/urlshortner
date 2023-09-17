-- Create the table to track visits
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    key varchar(10) REFERENCES urls(key), -- could have named it something better
    visited_at TIMESTAMPTZ DEFAULT NOW()
);
