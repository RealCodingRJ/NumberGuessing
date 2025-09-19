
type URI = String;

pub mod db {
    use crate::db::{URI};

    use mongodb::{Client, options::ClientOptions};

    pub async fn get_client_uri() -> URI {
        "mongodb:://localhost:27017".to_owned()
    }

    pub async fn get_client() -> Client {
        let db = ClientOptions::parse(get_client_uri().await);
        let client = Client::with_options(db.await.unwrap()).unwrap();

        client
    }
}