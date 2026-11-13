-- Create Enums
CREATE TYPE user_role AS ENUM ('ADMIN', 'TENANT', 'CLIENT');

-- Create Tables
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL,
    full_name VARCHAR(255) NOT NULL
);

CREATE TABLE houses (
    id SERIAL PRIMARY KEY,
    house_number VARCHAR(50) UNIQUE NOT NULL,
    address TEXT NOT NULL
);

CREATE TABLE floors (
    id SERIAL PRIMARY KEY,
    house_id INT REFERENCES houses(id),
    floor_no INT NOT NULL,
    capacity INT NOT NULL
);

CREATE TABLE tenants (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    floor_id INT REFERENCES floors(id),
    full_name VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    lease_start DATE,
    lease_end DATE,
    is_active BOOLEAN DEFAULT true
);

CREATE TABLE water_meters (
    id SERIAL PRIMARY KEY,
    meter_number VARCHAR(50) UNIQUE NOT NULL,
    house_id INT REFERENCES houses(id),
    tenant_id INT REFERENCES tenants(id),
    client_user_id INT REFERENCES users(id)
);

CREATE TABLE meter_readings (
    id SERIAL PRIMARY KEY,
    meter_id INT REFERENCES water_meters(id),
    reading_date DATE NOT NULL,
    previous_units INT NOT NULL,
    current_units INT NOT NULL,
    units_used INT GENERATED ALWAYS AS (current_units - previous_units) STORED,
    source VARCHAR(50) NOT NULL,
    image_path TEXT,
    is_billed BOOLEAN DEFAULT false
);

-- Seed Data (Password is 'password123')
INSERT INTO users (email, password_hash, role, full_name) VALUES 
('admin@regimeflyers.com', '$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$7Z1x8R5h4qA8z3y2w1v0u$', 'ADMIN', 'System Admin'),
('johndoe@example.com', '$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$7Z1x8R5h4qA8z3y2w1v0u$', 'TENANT', 'John Doe');

INSERT INTO houses (house_number, address) VALUES ('A-12', '123 Water Way');
INSERT INTO floors (house_id, floor_no, capacity) VALUES (1, 2, 4);
INSERT INTO tenants (user_id, floor_id, full_name, phone, is_active) VALUES (2, 1, 'John Doe', '+254700000000', true);
INSERT INTO water_meters (meter_number, house_id, tenant_id, client_user_id) VALUES ('WM-99821', 1, 1, 2);

INSERT INTO meter_readings (meter_id, reading_date, previous_units, current_units, source) VALUES 
(1, '2026-09-01', 100, 120, 'MANUAL'),
(1, '2026-10-01', 120, 145, 'OCR_SCAN');
