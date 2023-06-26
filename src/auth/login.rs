use types::{User};
use utils::{validate_login, clear_console, get_input};

use std::{thread, time};

#[derive(Debug, Clone)]
pub struct LoginForm;

impl LoginForm {
    /// Constructor of the LoginForm
    pub fn new(version: &'static str) -> User {
        loop {
            clear_console();
            println!("Yigit Bank! v{version}");
            println!("------------------------");
            println!("Username: ");
            let username_buffer = get_input();
            println!("\nPassword: ");
            let password_buffer = get_input();

            println!("\nİşlem yapılıyor...");
            if let Some(user) = validate_login(&username_buffer, &password_buffer) {
                clear_console();
                return user;
            } else {
                println!("\nYANLIŞ KULLANICI BİLGİLERİ GİRİLDİ!");
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
}
