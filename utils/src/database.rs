pub use mongodb::{Client, options::{ClientOptions, FindOptions}, Collection};
pub use mongodb::bson::{doc, Document, oid::{ObjectId}};
pub use futures::{TryStreamExt};
pub use dotenv::dotenv;
pub use std::env;
pub use types::{User, /* Balance */};

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pub current_db_name: String,
    pub current_cluster_name: String,
    pub current_cluster: Collection<User>,
}

impl DatabaseManager {
    pub async fn new() -> Option<DatabaseManager> {
        dotenv().ok();
        let db_uri: String = match env::var("DATABASE_URI") {
            Ok(val) => val,
            Err(_) => "mongodb://127.0.0.1:27017/".to_string(),
        };

        let db_name: String = match env::var("DB_NAME") {
            Ok(val) => val,
            Err(_) => "bank".to_string()
        };

        let cluster_name: String = match env::var("CLUSTER_NAME") {
            Ok(val) => val,
            Err(_) => "users".to_string()
        };

        // I can use unwrap_or but didnt want to use it :D

        let mut client_options = ClientOptions::parse(db_uri).await.ok()?;
        client_options.app_name = Some("YigitBank".to_string());

        let client = Client::with_options(client_options).ok()?;

        let database = client.database(&db_name);
        let collection = database.collection::<User>(&cluster_name);

        Some(Self {
            current_db_name: db_name,
            current_cluster_name: cluster_name,
            current_cluster: collection,
        })
    }
}
