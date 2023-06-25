pub use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub balance: Balance
}

impl User {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            balance: Balance::new()
        }
    }

    /// Returns a reference to the get username of this [`User`].
    pub fn get_username(&self) -> &str {
        self.username.as_ref()
    }

    /// Returns a reference to the get email of this [`User`].
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    /// Returns a reference to the get email of this [`User`].
    pub fn get_email(&self) -> &str {
        self.email.as_ref()
    }

    /// Returns a reference to the get password of this [`User`].
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    /// Returns a reference to the get password of this [`User`].
    pub fn get_password(&self) -> &str {
        self.password.as_ref()
    }

    /// Returns a reference to the get first name of this [`User`].
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    /// Returns a reference to the get first name of this [`User`].
    pub fn get_first_name(&self) -> &str {
        self.first_name.as_ref()
    }

    /// Returns a reference to the get last name of this [`User`].
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    /// Returns a reference to the get last name of this [`User`].
    pub fn get_last_name(&self) -> &str {
        self.last_name.as_ref()
    }

    /// Returns a reference to the get last name of this [`User`].
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    /// Returns a reference to the get balance of this [`User`].
    pub fn get_balance(&self) -> &Balance {
        &self.balance
    }

    /// Returns a reference to the get balance of this [`User`].
    pub fn set_balance(&mut self, balance: Balance) {
        self.balance = balance;
    }

    /// Returns a reference to the get balance of turkish lira this [`User`] | [`Balance`].
    pub fn get_balance_turkish_lira(&self) -> &f64 {
        &self.balance.turkish_lira
    }

    /// Returns a reference to the get balance of usd this [`User`] | [`Balance`].
    pub fn get_balance_usd(&self) -> &f64 {
        &self.balance.usd
    }

    /// Returns a reference to the get balance of euro this [`User`] | [`Balance`].
    pub fn get_balance_euro(&self) -> &f64 {
        &self.balance.euro
    }

    /// Returns a reference to the set balance of turkish lira this [`User`] | [`Balance`].
    pub fn set_balance_turkish_lira(&mut self, amount: f64) {
        self.balance.turkish_lira = amount;
    }

    /// Returns a reference to the set balance of usd this [`User`] | [`Balance`].
    pub fn set_balance_usd(&mut self, amount: f64) {
        self.balance.usd = amount;
    }

    /// Returns a reference to the set balance of euro this [`User`] | [`Balance`].
    pub fn set_balance_euro(&mut self, amount: f64) {
        self.balance.euro = amount;
    }

}


#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub turkish_lira: f64,
    pub usd: f64,
    pub euro: f64,
}

impl Balance {
    pub fn new() -> Self {
        Self {
            turkish_lira: 0.0,
            usd: 0.0,
            euro: 0.0,
        }
    }

    /// Returns the turkish lira of this User's [`Balance`].
    pub fn get_turkish_lira(&self) -> &f64 {
        &self.turkish_lira
    }

    /// Sets the turkish lira of this User's [`Balance`].
    pub fn set_turkish_lira(&mut self, turkish_lira: f64) {
        self.turkish_lira = turkish_lira;
    }

    /// Returns the get usd of this User's [`Balance`].
    pub fn get_usd(&self) -> &f64 {
        &self.usd
    }

    /// Sets the usd of this User's [`Balance`].
    pub fn set_usd(&mut self, usd: f64) {
        self.usd = usd;
    }

    /// Returns the euro of this User's [`Balance`].
    pub fn get_euro(&self) -> &f64 {
        &self.euro
    }

    /// Sets the euro of this  User's [`Balance`].
    pub fn set_euro(&mut self, euro: f64) {
        self.euro = euro;
    }
}
