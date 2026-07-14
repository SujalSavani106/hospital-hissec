// ============================================================
// HISSEC* — Hospital Information System Security
// Main entry point — Interactive Terminal UI
//
// Architecture:
//   Service → PEP → PDP → DYNAMO rules → Allow/Deny → Repository
// ============================================================

// Include DYNAMO-generated policy rules as a module.
// Lives at policy/generated/policy.rs — treated as auto-generated.
#[path = "../policy/generated/policy.rs"]
mod policy_generated;

mod config;
mod models;
mod storage;
mod policy;
mod services;
mod errors;
mod utils;

#[cfg(test)]
mod tests {
    mod policy_tests;
    mod login_tests;
    mod ehr_tests;
    mod sensor_tests;
    mod integration_tests;
}

use std::io::{self, Write};
use std::sync::Mutex;
use services::{AppState, AuthService, UserService, EhrService, SensorService};
use policy::pep::Pep;

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_selection(msg: &str, options: &[(String, String)]) -> String {
    if options.is_empty() {
        println!("{} (No options available)", msg);
        return String::new();
    }
    println!("\n{}", msg);
    for (i, (display, _)) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, display);
    }
    loop {
        print!("Select [1-{}]: ", options.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(idx) = input.trim().parse::<usize>() {
            if idx > 0 && idx <= options.len() {
                return options[idx - 1].1.clone();
            }
        }
        println!("Invalid selection. Try again.");
    }
}

fn print_help() {
    println!("\n--- System Entity Reference ---");
    println!("[System Notice: Seeded test accounts use default password 'password']");
    println!("Users:");
    println!("  ronak        (Manager)   [ward-icu]      | ID: u-mgr");
    println!("  sujal        (Clerk)     [ward-icu]      | ID: u-clk");
    println!("  devarsya     (Physician) [ward-icu]      | ID: u-ph1");
    println!("  jay          (Nurse)     [ward-icu]      | ID: u-nu1");
    println!("  kishan       (Paramedic) [ward-icu]      | ID: u-pa1");
    println!("  vivek        (Patient)   [ward-icu]      | ID: u-pt1");
    println!("  dr_grace     (Physician) [ward-surgery]  | ID: u-ph2");
    println!("  nurse_henry  (Nurse)     [ward-surgery]  | ID: u-nu2");
    println!("  dr_john      (Physician) [ward-internal] | ID: u-ph3");
    println!("  nurse_emma   (Nurse)     [ward-internal] | ID: u-nu3");
    println!("  patient_anna (Patient)   [ward-internal] | ID: u-pt2");
    println!("  dr_smith     (Physician) [ward-maternity]| ID: u-ph4");
    println!("  nurse_sophia (Nurse)     [ward-maternity]| ID: u-nu4");
    println!("  clerk_lisa   (Clerk)     [ward-maternity]| ID: u-clk2");
    println!("  patient_mia  (Patient)   [ward-maternity]| ID: u-pt3");
    println!("\nAssets:");
    println!("  EHRs: ehr-001 (ward-icu), ehr-002 (ward-surgery), ehr-003 (ward-internal), ehr-004 (ward-maternity)");
    println!("  Sensors: sen-icu-normal, sen-icu-critical, sen-surg-normal, sen-int-normal, sen-int-critical, sen-mat-normal");
    println!("---------------------------\n");
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║      HISSEC* — Hospital Information System Security      ║");
    println!("║                 Interactive Terminal UI                  ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let state_lock = Mutex::new(AppState::init());
    let pep = Pep::new();
    let mut current_session: Option<String> = None;
    let mut current_user: Option<String> = None;
    let mut current_role: Option<String> = None;

    print_help();

    loop {
        println!("\n=================================");
        if let Some(ref u) = current_user {
            println!("[USER] Logged in as: {} (Role: {})", u, current_role.as_deref().unwrap_or(""));
            println!(" 1. Read EHR");
            println!(" 2. Create EHR");
            println!(" 3. Delete EHR");
            println!(" 4. Fetch Sensor");
            println!(" 5. Add User");
            println!(" 6. Remove User");
            println!(" 7. Assign Role");
            println!(" 8. Change Ward");
            println!(" 9. Logout");
            println!(" h. Show Test Data");
            println!(" 0. Exit");
        } else {
            println!("[USER] Not logged in");
            println!(" 1. Login");
            println!(" h. Show Test Data");
            println!(" 0. Exit");
        }
        
        let choice = prompt("\nSelect an option > ");
        
        if choice == "h" {
            print_help();
            continue;
        } else if choice == "0" {
            println!("Goodbye!");
            break;
        }

        if current_session.is_none() {
            match choice.as_str() {
                "1" => {
                    let user_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.users.all().iter().map(|u| (format!("{} ({})", u.username, u.id), u.username.clone())).collect()
                    };
                    let username = prompt_selection("Select Username to Login:", &user_opts);
                    if username.is_empty() { continue; }

                    let password = prompt("Password: ");
                    
                    let role_opts = vec![
                        ("Patient".to_string(), "patient".to_string()),
                        ("Physician".to_string(), "physician".to_string()),
                        ("Nurse".to_string(), "nurse".to_string()),
                        ("Paramedic".to_string(), "paramedic".to_string()),
                        ("Manager".to_string(), "manager".to_string()),
                        ("Clerk".to_string(), "clerk".to_string()),
                    ];
                    let role = prompt_selection("Select Role to assume:", &role_opts);

                    let mut state = state_lock.lock().unwrap();
                    match AuthService::login(&mut state, &username, &password, &role, &pep) {
                        Ok(subject) => {
                            println!("\n[SUCCESS] Login successful! Session ID: {}", subject.session_id);
                            current_session = Some(subject.session_id);
                            current_user = Some(username);
                            current_role = Some(role);
                        }
                        Err(e) => println!("\n[FAIL] Login failed: {}", e),
                    }
                }
                _ => println!("Invalid choice."),
            }
        } else {
            let sid = current_session.clone().unwrap();
            match choice.as_str() {
                "1" => {
                    let ehr_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.ehrs.all().iter().map(|e| (format!("{} (in {})", e.id, e.ward_id), e.id.clone())).collect()
                    };
                    let ehr_id = prompt_selection("Select EHR to read:", &ehr_opts);
                    if ehr_id.is_empty() { continue; }

                    let state = state_lock.lock().unwrap();
                    match EhrService::read_ehr(&state, &sid, &ehr_id, &pep) {
                        Ok(ehr) => println!("\n[SUCCESS] EHR Data:\n   ID: {}\n   Ward: {}\n   Notes: {:?}", ehr.id, ehr.ward_id, ehr.notes),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "2" => {
                    let ward_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.wards.all().iter().map(|w| (w.name.clone(), w.id.clone())).collect()
                    };
                    let ward_id = prompt_selection("Select Ward to create EHR in:", &ward_opts);
                    if ward_id.is_empty() { continue; }

                    let mut user_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.users.all().iter().map(|u| (format!("{} ({})", u.username, u.id), u.id.clone())).collect()
                    };
                    user_opts.insert(0, ("None (Skip)".to_string(), String::new()));
                    
                    let patient_id_sel = prompt_selection("Select Patient (optional):", &user_opts);
                    let p_id = if patient_id_sel.is_empty() { None } else { Some(patient_id_sel) };
                    
                    let notes = prompt("Notes: ");
                    let mut state = state_lock.lock().unwrap();
                    match EhrService::create_ehr(&mut state, &sid, &ward_id, p_id, Some(notes), &pep) {
                        Ok(ehr) => println!("\n[SUCCESS] Created EHR with ID: {}", ehr.id),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "3" => {
                    let ehr_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.ehrs.all().iter().map(|e| (format!("{} (in {})", e.id, e.ward_id), e.id.clone())).collect()
                    };
                    let ehr_id = prompt_selection("Select EHR to delete:", &ehr_opts);
                    if ehr_id.is_empty() { continue; }

                    let mut state = state_lock.lock().unwrap();
                    match EhrService::delete_ehr(&mut state, &sid, &ehr_id, &pep) {
                        Ok(_) => println!("\n[SUCCESS] EHR deleted."),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "4" => {
                    let sensor_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.sensors.all().iter().map(|s| (format!("{} (in {})", s.id, s.ward_id), s.id.clone())).collect()
                    };
                    let sensor_id = prompt_selection("Select Sensor to fetch:", &sensor_opts);
                    if sensor_id.is_empty() { continue; }

                    let state = state_lock.lock().unwrap();
                    match SensorService::fetch_sensor(&state, &sid, &sensor_id, &pep) {
                        Ok(sensor) => println!("\n[SUCCESS] Sensor Data:\n   ID: {}\n   Ward: {}\n   Type: {:?}\n   Desc: {:?}", sensor.id, sensor.ward_id, sensor.sensor_type, sensor.description),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "5" => {
                    let username = prompt("New Username: ");
                    let password = prompt("Password: ");
                    
                    let ward_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.wards.all().iter().map(|w| (w.name.clone(), w.id.clone())).collect()
                    };
                    let ward_id = prompt_selection("Select initial Ward:", &ward_opts);
                    if ward_id.is_empty() { continue; }

                    let mut state = state_lock.lock().unwrap();
                    match UserService::add_user(&mut state, &sid, &username, &password, &ward_id, &pep) {
                        Ok(user) => println!("\n[SUCCESS] Created user with ID: {}", user.id),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "6" => {
                    let user_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.users.all().iter().map(|u| (format!("{} ({})", u.username, u.id), u.id.clone())).collect()
                    };
                    let user_id = prompt_selection("Select User to remove:", &user_opts);
                    if user_id.is_empty() { continue; }

                    let mut state = state_lock.lock().unwrap();
                    match UserService::remove_user(&mut state, &sid, &user_id, &pep) {
                        Ok(_) => println!("\n[SUCCESS] User removed."),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "7" => {
                    let user_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.users.all().iter().map(|u| (format!("{} ({})", u.username, u.id), u.id.clone())).collect()
                    };
                    let user_id = prompt_selection("Select Target User:", &user_opts);
                    if user_id.is_empty() { continue; }

                    let role_opts = vec![
                        ("Patient".to_string(), "patient".to_string()),
                        ("Physician".to_string(), "physician".to_string()),
                        ("Nurse".to_string(), "nurse".to_string()),
                        ("Paramedic".to_string(), "paramedic".to_string()),
                        ("Manager".to_string(), "manager".to_string()),
                        ("Clerk".to_string(), "clerk".to_string()),
                    ];
                    let new_role = prompt_selection("Select Role to assign:", &role_opts);

                    let mut state = state_lock.lock().unwrap();
                    match UserService::assign_role(&mut state, &sid, &user_id, &new_role, &pep) {
                        Ok(_) => println!("\n[SUCCESS] Role assigned."),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "8" => {
                    let user_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.users.all().iter().map(|u| (format!("{} ({})", u.username, u.id), u.id.clone())).collect()
                    };
                    let user_id = prompt_selection("Select Target User:", &user_opts);
                    if user_id.is_empty() { continue; }

                    let ward_opts: Vec<(String, String)> = {
                        let state = state_lock.lock().unwrap();
                        state.wards.all().iter().map(|w| (w.name.clone(), w.id.clone())).collect()
                    };
                    let new_ward = prompt_selection("Select New Ward:", &ward_opts);
                    if new_ward.is_empty() { continue; }

                    let mut state = state_lock.lock().unwrap();
                    match UserService::change_ward(&mut state, &sid, &user_id, &new_ward, &pep) {
                        Ok(_) => println!("\n[SUCCESS] Ward changed."),
                        Err(e) => println!("\n[FAIL] Failed: {}", e),
                    }
                }
                "9" => {
                    let mut state = state_lock.lock().unwrap();
                    let _ = AuthService::logout(&mut state, &sid, &pep);
                    current_session = None;
                    current_user = None;
                    current_role = None;
                    println!("\n[SUCCESS] Logged out.");
                }
                _ => println!("\n[FAIL] Invalid choice."),
            }
        }
    }
}
