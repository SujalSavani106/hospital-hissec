-- ============================================================
-- HISSEC* Seed Data
-- Scenario matching the permission matrix tests
-- ============================================================

-- Wards
INSERT INTO wards VALUES ('ward-icu',      'ICU');
INSERT INTO wards VALUES ('ward-surgery',  'Surgery');
INSERT INTO wards VALUES ('ward-internal', 'Internal');
INSERT INTO wards VALUES ('ward-maternity','Maternity');

-- Roles
INSERT INTO roles VALUES ('role-patient',   'patient');
INSERT INTO roles VALUES ('role-physician', 'physician');
INSERT INTO roles VALUES ('role-nurse',     'nurse');
INSERT INTO roles VALUES ('role-paramedic', 'paramedic');
INSERT INTO roles VALUES ('role-manager',   'manager');
INSERT INTO roles VALUES ('role-clerk',     'clerk');

-- Users (password_hash = sha256("password"))
-- Manager
INSERT INTO users VALUES ('u-mgr',  'mgr_alice',   'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Clerk
INSERT INTO users VALUES ('u-clk',  'clk_bob',     'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Physician (ICU)
INSERT INTO users VALUES ('u-ph1',  'dr_carter',   'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Nurse (ICU)
INSERT INTO users VALUES ('u-nu1',  'nurse_dana',  'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Paramedic
INSERT INTO users VALUES ('u-pa1',  'para_evan',   'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Patient (ICU)
INSERT INTO users VALUES ('u-pt1',  'pat_frank',   'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-icu');
-- Physician (Surgery) - for cross-ward tests
INSERT INTO users VALUES ('u-ph2',  'dr_grace',    'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-surgery');
-- Nurse (Surgery) - for cross-ward tests
INSERT INTO users VALUES ('u-nu2',  'nurse_henry', 'SHA256:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8', 'ward-surgery');

-- User-Role assignments
INSERT INTO user_roles VALUES ('u-mgr', 'role-manager');
INSERT INTO user_roles VALUES ('u-clk', 'role-clerk');
INSERT INTO user_roles VALUES ('u-ph1', 'role-physician');
INSERT INTO user_roles VALUES ('u-nu1', 'role-nurse');
INSERT INTO user_roles VALUES ('u-pa1', 'role-paramedic');
INSERT INTO user_roles VALUES ('u-pt1', 'role-patient');
INSERT INTO user_roles VALUES ('u-ph2', 'role-physician');
INSERT INTO user_roles VALUES ('u-nu2', 'role-nurse');

-- EHRs
INSERT INTO ehrs VALUES ('ehr-001', 'ward-icu',     'u-pt1', 'ICU patient EHR');
INSERT INTO ehrs VALUES ('ehr-002', 'ward-surgery',  NULL,    'Surgery department EHR');

-- Sensors
INSERT INTO sensors VALUES ('sen-icu-normal',   'ward-icu',     'Normal',   'ICU Temperature sensor');
INSERT INTO sensors VALUES ('sen-icu-critical', 'ward-icu',     'Critical', 'ICU Ventilator sensor');
INSERT INTO sensors VALUES ('sen-surg-normal',  'ward-surgery',  'Normal',   'Surgery BP sensor');
