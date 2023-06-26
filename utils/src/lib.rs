pub use mongodb::{Client, options::ClientOptions};
pub use types::{User, Balance};

pub mod database;

/// Validates login creadentials from MongoDB Database.
pub fn validate_login(_username: &str, _password: &str) -> Option<User> {

    None
}

/// It clears console screen.
pub fn clear_console() -> () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
