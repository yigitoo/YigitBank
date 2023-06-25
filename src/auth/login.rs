use types::{User, Balance};
use utils::{validate_login};

use std::io::stdin as input;

#[derive(Debug, Clone)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

impl LoginForm {
    /// Constructor of the LoginForm
    pub fn new() -> Option<User> {
        let mut username_buffer = String::new();
        let mut password_buffer = String::new();

        loop {
            print!("Username: ");
            input().read_line(&username_buffer).unwrap_or("")
            print!("Password: ");
            input().read_line(&mut password_buffer).unwrap_or("")

            if username_buffer != "" && password_buffer != "" {
                break
            }
        }

        if let Some(user) = ValidateLogin(username_buffer, password_buffer) {
            return Some(user);
        }
        None
    }
}

impl Default for LoginForm {
    fn default() -> Self {
        Self::new()
    }
}
