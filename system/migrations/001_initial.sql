-- Enable UUID extension if not already present
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 1. Users Table
CREATE TABLE IF NOT EXISTS users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role          VARCHAR(20) NOT NULL CHECK (role IN ('ADMIN', 'TENANT', 'CLIENT')),
    full_name     VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Houses Table
CREATE TABLE IF NOT EXISTS houses (
    id           SERIAL PRIMARY KEY,
    house_number VARCHAR(50) UNIQUE NOT NULL,
    address      TEXT,
    created_at   TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Floors Table (Tracks how many tenants a floor holds)
CREATE TABLE IF NOT EXISTS floors (
    id        SERIAL PRIMARY KEY,
    house_id  INT NOT NULL REFERENCES houses(id) ON DELETE CASCADE,
    floor_no  INT NOT NULL,
    capacity  INT NOT NULL DEFAULT 1,
    UNIQUE (house_id, floor_no)
);

-- 4. Tenants Table
CREATE TABLE IF NOT EXISTS tenants (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID UNIQUE REFERENCES users(id) ON DELETE SET NULL,
    floor_id    INT NOT NULL REFERENCES floors(id) ON DELETE CASCADE,
    full_name   VARCHAR(255) NOT NULL,
    phone       VARCHAR(20),
    lease_start DATE,
    lease_end   DATE,
    is_active   BOOLEAN DEFAULT TRUE
);

-- 5. Water Meters Table
CREATE TABLE IF NOT EXISTS water_meters (
    id                SERIAL PRIMARY KEY,
    meter_number      VARCHAR(100) UNIQUE NOT NULL,
    house_id          INT REFERENCES houses(id) ON DELETE CASCADE,
    tenant_id         UUID REFERENCES tenants(id) ON DELETE SET NULL,
    client_user_id    UUID REFERENCES users(id) ON DELETE SET NULL,
    installation_date DATE
);

-- 6. Meter Readings Table
CREATE TABLE IF NOT EXISTS meter_readings (
    id             SERIAL PRIMARY KEY,
    meter_id       INT NOT NULL REFERENCES water_meters(id) ON DELETE CASCADE,
    reading_date   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    previous_units NUMERIC(10,2) NOT NULL,
    current_units  NUMERIC(10,2) NOT NULL,
    units_used     NUMERIC(10,2) GENERATED ALWAYS AS (current_units - previous_units) STORED,
    source         VARCHAR(20) NOT NULL CHECK (source IN ('MANUAL', 'OCR_SCAN')),
    image_path     TEXT,
    is_billed      BOOLEAN DEFAULT FALSE,
    created_by     UUID REFERENCES users(id)
);

-- Index for faster queries
CREATE INDEX IF NOT EXISTS idx_readings_meter ON meter_readings(meter_id);
CREATE INDEX IF NOT EXISTS idx_tenants_user ON tenants(user_id);
CREATE INDEX IF NOT EXISTS idx_meters_client ON water_meters(client_user_id);
