mod esi;

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod model;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref APP_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref APP_EMAIL: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

pub fn initialize_eve_esi(application_name: String, application_email: String) {
    let mut app_name = APP_NAME.lock().unwrap();
    *app_name = application_name;

    let mut app_email = APP_EMAIL.lock().unwrap();
    *app_email = application_email;
}
