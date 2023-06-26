#[allow(unused_imports, non_snake_case)]
pub use mongodb::{sync::{Client, Collection, Database}, options::{ClientOptions}};
pub use mongodb::bson::{doc, Document, oid::{ObjectId}};
pub use dotenv::dotenv;
pub use std::env;
pub use types::{User, Balance };

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pub current_db_name: String,
    pub current_collection_name: String,
    pub database: Database,
}

impl DatabaseManager {
    pub fn new() -> Self {
        dotenv().ok();
        let db_uri: String = match env::var("DB_URI") {
            Ok(val) => val,
            Err(_) => "mongodb://127.0.0.1:27017/".to_string(),
        };

        let db_name: String = match env::var("DB_NAME") {
            Ok(val) => val,
            Err(_) => "bank".to_string()
        };

        let collection_name: String = match env::var("collection_NAME") {
            Ok(val) => val,
            Err(_) => "users".to_string()
        };

        let client = Client::with_uri_str(db_uri.as_str()).unwrap();
        let database = client.database(&db_name);
        Self {
            current_db_name: db_name,
            current_collection_name: collection_name,
            database: database,
        }
    }
}
