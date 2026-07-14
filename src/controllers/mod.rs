pub mod auth_controller;
pub mod user_controller;
pub mod ehr_controller;
pub mod sensor_controller;

pub use auth_controller::{login, logout};
pub use user_controller::{add_user, remove_user, assign_role, change_ward};
pub use ehr_controller::{read_ehr, create_ehr, delete_ehr};
pub use sensor_controller::fetch_sensor;
