pub mod login;

pub mod Auth {
    pub use types::{User};
    pub use super::login::LoginForm as LoginForm;

    pub fn new() -> User {
        let user = LoginForm::new();
        return user;
    }
}
