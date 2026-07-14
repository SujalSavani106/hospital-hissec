-- ============================================================
-- HISSEC* Database Schema
-- Hospital Information System Security
-- ============================================================

-- Wards (static)
CREATE TABLE IF NOT EXISTS wards (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id          TEXT PRIMARY KEY,
    username    TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    ward_id     TEXT NOT NULL REFERENCES wards(id)
);

-- Roles (static)
CREATE TABLE IF NOT EXISTS roles (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE  -- patient, physician, nurse, paramedic, manager, clerk
);

-- User-Role assignments
CREATE TABLE IF NOT EXISTS user_roles (
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id     TEXT NOT NULL REFERENCES roles(id),
    PRIMARY KEY (user_id, role_id)
);

-- EHR objects
CREATE TABLE IF NOT EXISTS ehrs (
    id          TEXT PRIMARY KEY,
    ward_id     TEXT NOT NULL REFERENCES wards(id),
    patient_id  TEXT REFERENCES users(id),
    notes       TEXT
);

-- Sensors
CREATE TABLE IF NOT EXISTS sensors (
    id          TEXT PRIMARY KEY,
    ward_id     TEXT NOT NULL REFERENCES wards(id),
    sensor_type TEXT NOT NULL CHECK(sensor_type IN ('Normal', 'Critical')),
    description TEXT
);

-- Active subjects (sessions)
CREATE TABLE IF NOT EXISTS subjects (
    session_id  TEXT PRIMARY KEY,
    user_id     TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    active_role TEXT NOT NULL REFERENCES roles(name),
    created_at  TEXT NOT NULL
);

-- Audit log (every PDP decision, per rule G7)
CREATE TABLE IF NOT EXISTS audit_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp   TEXT NOT NULL,
    session_id  TEXT,
    username    TEXT,
    active_role TEXT,
    operation   TEXT NOT NULL,
    target_id   TEXT,
    decision    TEXT NOT NULL CHECK(decision IN ('ALLOW', 'DENY')),
    reason      TEXT
);
