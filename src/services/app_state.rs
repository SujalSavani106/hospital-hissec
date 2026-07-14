use std::sync::Mutex;
use crate::storage::{
    UserRepository, SubjectRepository, EhrRepository, SensorRepository, WardRepository,
};
use crate::models::{Ward, User, Sensor, SensorType};
use crate::utils::helpers::{hash_password, generate_id};

/// Shared in-memory application state, protected by a Mutex.
/// Passed as `web::Data<Mutex<AppState>>` to all Actix handlers.
pub struct AppState {
    pub users:    UserRepository,
    pub subjects: SubjectRepository,
    pub ehrs:     EhrRepository,
    pub sensors:  SensorRepository,
    pub wards:    WardRepository,
}

impl AppState {
    /// Initialize the state with seed data matching the test scenarios.
    pub fn init() -> Self {
        let wards = WardRepository::new().seed(vec![
            Ward::new("ward-icu".into(),       "ICU".into()),
            Ward::new("ward-surgery".into(),   "Surgery".into()),
            Ward::new("ward-internal".into(),  "Internal".into()),
            Ward::new("ward-maternity".into(), "Maternity".into()),
        ]);

        let mut users = UserRepository::new();
        let mut sensors = SensorRepository::new();
        let mut ehrs = EhrRepository::new();

        // Seed users
        use crate::models::RoleKind::*;
        let seed = vec![
            ("u-mgr",  "ronak",   "ward-icu",      vec![Manager]),
            ("u-clk",  "sujal",     "ward-icu",      vec![Clerk]),
            ("u-ph1",  "devarsya",   "ward-icu",      vec![Physician]),
            ("u-nu1",  "jay",  "ward-icu",      vec![Nurse]),
            ("u-pa1",  "kishan",   "ward-icu",      vec![Paramedic]),
            ("u-pt1",  "vivek",   "ward-icu",      vec![Patient]),
            ("u-ph2",  "dr_grace",    "ward-surgery",  vec![Physician]),
            ("u-nu2",  "nurse_henry", "ward-surgery",  vec![Nurse]),
            // New Internal Ward Users
            ("u-ph3",  "dr_john",     "ward-internal", vec![Physician]),
            ("u-nu3",  "nurse_emma",  "ward-internal", vec![Nurse]),
            ("u-pt2",  "patient_anna","ward-internal", vec![Patient]),
            // New Maternity Ward Users
            ("u-ph4",  "dr_smith",    "ward-maternity",vec![Physician]),
            ("u-nu4",  "nurse_sophia","ward-maternity",vec![Nurse]),
            ("u-clk2", "clerk_lisa",  "ward-maternity",vec![Clerk]),
            ("u-pt3",  "patient_mia", "ward-maternity",vec![Patient]),
        ];
        for (id, uname, ward, roles) in seed {
            let mut u = User::new(
                id.into(), uname.into(),
                hash_password("password"), ward.into(),
            );
            u.roles = roles;
            users.save(u);
        }

        // Seed sensors
        sensors.save(crate::models::Sensor::new(
            "sen-icu-normal".into(), "ward-icu".into(),
            SensorType::Normal, Some("Continuous Core Temperature Monitor (Esophageal Probe)".into()),
        ));
        sensors.save(crate::models::Sensor::new(
            "sen-icu-critical".into(), "ward-icu".into(),
            SensorType::Critical, Some("PB 980 Mechanical Ventilator (Critical Flow Data)".into()),
        ));
        sensors.save(crate::models::Sensor::new(
            "sen-surg-normal".into(), "ward-surgery".into(),
            SensorType::Normal, Some("Invasive Arterial Blood Pressure Line".into()),
        ));
        sensors.save(crate::models::Sensor::new(
            "sen-int-normal".into(), "ward-internal".into(),
            SensorType::Normal, Some("Non-Invasive Blood Pressure (NIBP) Cuff".into()),
        ));
        sensors.save(crate::models::Sensor::new(
            "sen-int-critical".into(), "ward-internal".into(),
            SensorType::Critical, Some("Continuous Telemetry ECG Monitor".into()),
        ));
        sensors.save(crate::models::Sensor::new(
            "sen-mat-normal".into(), "ward-maternity".into(),
            SensorType::Normal, Some("Fetal Cardiotocograph (CTG) Monitor".into()),
        ));

        // Seed EHRs
        ehrs.save(crate::models::Ehr::new(
            "ehr-001".into(), "ward-icu".into(),
            Some("u-pt1".into()), Some("Admission Diagnosis: Sepsis secondary to pneumonia. Patient intubated. Vitals unstable.".into()),
        ));
        ehrs.save(crate::models::Ehr::new(
            "ehr-002".into(), "ward-surgery".into(),
            None, Some("Pre-operative assessment for Appendectomy. Patient NPO since midnight. No known allergies.".into()),
        ));
        ehrs.save(crate::models::Ehr::new(
            "ehr-003".into(), "ward-internal".into(),
            Some("u-pt2".into()), Some("Progress Note: Patient admitted for DKA management. Insulin drip initiated. Potassium replacement ongoing.".into()),
        ));
        ehrs.save(crate::models::Ehr::new(
            "ehr-004".into(), "ward-maternity".into(),
            Some("u-pt3".into()), Some("G3P2 patient at 38 weeks gestation. Admitted for active labor. Epidural placed at 14:00.".into()),
        ));

        Self { users, subjects: SubjectRepository::new(), ehrs, sensors, wards }
    }
}
