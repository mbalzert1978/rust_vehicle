CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    manufacturer TEXT,
    manufacturing_year INTEGER,
    is_driveable BOOLEAN NOT NULL DEFAULT false,
    body JSONB NOT NULL DEFAULT '{}'
);
