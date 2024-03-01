mod esi;

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod model;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref USER_AGENT: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

pub fn initialize_eve_esi(application_name: String, application_email: String) {
    let mut user_agent = USER_AGENT.lock().unwrap();
    *user_agent = format!("{} ({})", application_name, application_email);
}
