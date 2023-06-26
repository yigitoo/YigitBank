pub mod login;

#[allow(non_snake_case)]
pub mod Auth {
    pub use types::{User};
    pub use super::login::LoginForm as LoginForm;

    pub fn new(version: &'static str) -> User {
        let user = LoginForm::new(version);
        return user;
    }
}
