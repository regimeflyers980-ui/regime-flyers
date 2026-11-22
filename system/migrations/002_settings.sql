CREATE TABLE IF NOT EXISTS settings (
    key VARCHAR(50) PRIMARY KEY,
    value VARCHAR(255) NOT NULL
);


-- Insert the default rate ($1.50 per unit)
INSERT INTO settings (key, value) VALUES ('water_rate', '1.50') ON CONFLICT DO NOTHING;
