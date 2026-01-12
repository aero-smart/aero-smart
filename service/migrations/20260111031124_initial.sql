-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create experiments table
CREATE TABLE experiments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create imu_records table
CREATE TABLE imu_records (
    id SERIAL PRIMARY KEY,
    session UUID NOT NULL REFERENCES experiments(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    accel_x REAL NOT NULL,
    accel_y REAL NOT NULL,
    accel_z REAL NOT NULL,
    gyro_x REAL NOT NULL,
    gyro_y REAL NOT NULL,
    gyro_z REAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT imu_session_timestamp_unique UNIQUE (session, timestamp)
);

-- Create pitot_airspeed_records table
CREATE TABLE pitot_airspeed_records (
    id SERIAL PRIMARY KEY,
    session UUID NOT NULL REFERENCES experiments(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    splitter_left REAL NOT NULL CHECK (splitter_left >= 0),
    splitter_right REAL NOT NULL CHECK (splitter_right >= 0),
    static_port REAL NOT NULL CHECK (static_port >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pitot_session_timestamp_unique UNIQUE (session, timestamp)
);

-- Create acoustic_records table
CREATE TABLE acoustic_records (
    id SERIAL PRIMARY KEY,
    session UUID NOT NULL REFERENCES experiments(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    overall_spl REAL NOT NULL CHECK (overall_spl >= 0),
    peak_frequency REAL NOT NULL CHECK (peak_frequency >= 0),
    spectral_shape vector(32) NOT NULL,
    turbulence_index REAL NOT NULL CHECK (turbulence_index >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT acoustic_session_timestamp_unique UNIQUE (session, timestamp)
);

-- Create indexes for better query performance
CREATE INDEX idx_imu_session ON imu_records(session);
CREATE INDEX idx_imu_timestamp ON imu_records(timestamp);
CREATE INDEX idx_imu_session_timestamp ON imu_records(session, timestamp DESC);

CREATE INDEX idx_pitot_session ON pitot_airspeed_records(session);
CREATE INDEX idx_pitot_timestamp ON pitot_airspeed_records(timestamp);
CREATE INDEX idx_pitot_session_timestamp ON pitot_airspeed_records(session, timestamp DESC);

CREATE INDEX idx_acoustic_session ON acoustic_records(session);
CREATE INDEX idx_acoustic_timestamp ON acoustic_records(timestamp);
CREATE INDEX idx_acoustic_session_timestamp ON acoustic_records(session, timestamp DESC);

-- Create HNSW index for vector similarity search (better performance than IVFFlat for most cases)
CREATE INDEX idx_acoustic_spectral_shape ON acoustic_records 
USING hnsw (spectral_shape vector_cosine_ops);

-- Create index on experiment name for faster lookups
CREATE INDEX idx_experiments_name ON experiments(name);

-- Add trigger for updated_at on experiments
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_experiments_updated_at BEFORE UPDATE ON experiments
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Add comments for documentation
COMMENT ON TABLE experiments IS 'Stores experiment metadata';
COMMENT ON TABLE imu_records IS 'Stores IMU (Inertial Measurement Unit) sensor data - accelerometer and gyroscope readings';
COMMENT ON TABLE pitot_airspeed_records IS 'Stores pitot tube airspeed sensor data from left/right splitter and static port';
COMMENT ON TABLE acoustic_records IS 'Stores acoustic measurement data with vector embeddings for spectral analysis';

COMMENT ON COLUMN acoustic_records.spectral_shape IS '32-dimensional vector embedding of spectral data for similarity search';
COMMENT ON COLUMN acoustic_records.overall_spl IS 'Overall Sound Pressure Level';
COMMENT ON COLUMN acoustic_records.peak_frequency IS 'Dominant frequency in the acoustic signal';
COMMENT ON COLUMN acoustic_records.turbulence_index IS 'Calculated turbulence index from acoustic data';