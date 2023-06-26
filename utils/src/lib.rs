pub use mongodb::{bson::doc};
pub use types::{User, Balance};
pub mod database;

use database::{DatabaseManager};

/// Validates login creadentials from MongoDB Database.
pub fn validate_login(username: &str, password: &str) -> Option<User> {
    let db_manager = DatabaseManager::new();
    let user = db_manager.database.collection("users").find_one(doc! {
        "username": username,
        "password": password
    }, None).unwrap();
    return user;
}

/// It clears console screen.
pub fn clear_console() -> () {
    let term = console::Term::stdout();
    term.clear_screen().expect("failed clearing console screen.")
}

/// Gets input from user.
pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    return input.trim().to_string();
}
