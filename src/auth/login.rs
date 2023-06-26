use types::{User};
use utils::{validate_login, clear_console};

use std::io::stdin as input;

pub struct LoginForm;

impl LoginForm {
    /// Constructor of the LoginForm
    pub fn new() -> User {
        let mut username_buffer = String::new();
        let mut password_buffer = String::new();

        loop {
            clear_console();

            print!("Username: ");
            input().read_line(&mut username_buffer).unwrap();

            print!("Password: ");
            input().read_line(&mut password_buffer).unwrap();

            if let Some(user) = validate_login(&username_buffer, &password_buffer) {
                clear_console();
                return user;
            } else {
                continue;
            }
        }
    }
}
