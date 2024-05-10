-- Add migration script here
DROP TABLE IF EXISTS vehicles;
CREATE TABLE vehicles (
    id UUID PRIMARY KEY NULL NULL,
    name TEXT NOT NULL,
    manufacturer TEXT,
    manufacturing_year INTEGER,
    is_driveable BOOLEAN NOT NULL DEFAULT false,
    body JSONB NOT NULL DEFAULT '{}'
);