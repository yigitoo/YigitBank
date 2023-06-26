static VERSION: &'static str = "0.0.1";
mod auth;

pub use auth::{Auth, login::LoginForm};

fn main() -> () {
    println!("Yigit Bank! {VERSION}");

    let auth_user = Auth::new();
    let bank_app_session = Bank::create_session(auth_user);

    return;
}
