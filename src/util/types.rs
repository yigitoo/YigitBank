use serde::serialize::{Deserialize, Serialize};

mod types {
    #[derive(Debug, Serialize, Deserialize)]
    struct Customer {
        name: String,
        address: String,
        phone: String,
        wallet: Wallet,
        // TODO: implement it as uuid4
        token: String
    }


    #[derive(Debug, Serialize, Deserialize)]
    struct Wallet {
        euro: f32,
        dollar: f32,
        turkish_lira: f32,
        transactions: Vec<Transaction>
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Transaction {
        amount: f32,
        currency: String
    }

    enum Currency {
        Euro,
        Dollar,
        TurkishLira
    }
}
