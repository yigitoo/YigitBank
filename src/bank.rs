#[allow(unused_imports)]
pub use types::{User, Balance, Currencies};
pub use types::Currencies::{TurkishLira, Dollar, Euro};
pub use utils::{clear_console, get_input};

use std::{thread, time};

#[derive(Debug, Clone)]
pub struct Bank {
    pub session_user: User,
}

impl Bank {
    pub fn new(session_user: User) -> Self {
        Self { session_user: session_user }
    }

    pub fn create_session(session_user: User) -> Self {
        Self::new(session_user)
    }

    pub fn run(&self) -> () {
        loop {
            let choosed_option = self.choose_options();
            match choosed_option {
                1 => {
                    self.deposit();
                },
                2 => {
                    self.withdraw();
                },
                3 => {
                    self.transfer();
                }
                4 => {
                    self.profile();
                },
                5 => {
                    self.exit();
                },
                _ => {
                    println!("Invalid option!");
                }
            }
        }
    }

    pub fn choose_options(&self) -> u8 {
        clear_console();
        println!("Yigit Bank (love Koc's)");
        println!("-----------------------");
        println!("1. Deposit Money");
        println!("2. Withdraw Money");
        println!("3. Transfer Money to Someone");
        println!("4. Profile Informations");
        println!("5. Exit");
        println!("-----------------------");
        println!("Choose an option:");
        let input = get_input();
        let parse_to_int = input.parse::<u8>();
        match parse_to_int {
            Ok(x) => x,
            Err(_) => {
                0
            }
        }
    }

    pub fn deposit(&self) -> () {
        clear_console();
        println!("Yatıracağın miktarı gir");

    }

    pub fn withdraw(&self) -> () {
    }

    pub fn transfer(&self) -> () {
    }

    pub fn profile(&self) {
        let _balance = self.session_user.get_balance();
    }

    pub fn exit(&self) -> () {
        println!("\nBye!");

        thread::sleep(time::Duration::from_secs(2));
        std::process::exit(0);
    }


    pub fn get_balance(&self) -> &Balance {
        return self.session_user.get_balance()
    }

    pub fn set_balance(&mut self, currency_type: Currencies, amount: f64) -> () {
        match currency_type {
            TurkishLira => self.session_user.set_balance_turkish_lira(amount),
            Dollar => self.session_user.set_balance_dollar(amount),
            Euro => self.session_user.set_balance_dollar(amount),
        };
    }
}
