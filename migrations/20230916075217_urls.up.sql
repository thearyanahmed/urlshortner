-- Create the table to store original URLs
CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    original_url TEXT NOT NULL,
    key TEXT UNIQUE NOT NULL, -- @TODO change
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_key ON urls (key);
