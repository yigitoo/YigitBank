static VERSION: &'static str = "0.0.1";
mod auth;
mod bank;

pub use bank::Bank;
pub use auth::{Auth, login::LoginForm};
pub use utils::clear_console;

fn main() -> std::io::Result<()> {
    // authentication screen.
    let auth_user = Auth::new(VERSION);

    // Logged user session.
    let app = Bank::new(auth_user);

    app.run();
    clear_console();
    return Ok(());
}
