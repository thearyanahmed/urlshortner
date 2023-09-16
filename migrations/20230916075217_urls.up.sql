-- Create the table to store original URLs
CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    original_url varchar(2000) NOT NULL,
    key varchar(10) UNIQUE NOT NULL, -- @TODO change
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_key ON urls (key);
