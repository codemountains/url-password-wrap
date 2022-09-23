use std::env;
use std::sync::Arc;

use mongodb::{Client, Database};

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

const URL: &str = "DATABASE_URL";
const DB_NAME: &str = "URL_WRAP_DB_NAME";

impl Db {
    pub async fn new() -> Db {
        let uri = env::var(URL).expect(undefined_msg(URL).as_str());
        let db_name = env::var(DB_NAME).expect(undefined_msg(DB_NAME).as_str());

        let client = Client::with_uri_str(&uri)
            .await
            .expect("Could not connect to MongoDB.");
        let db = client.database(&db_name);

        Db(Arc::new(db))
    }
}

fn undefined_msg(subject: &str) -> String {
    format!("{} is undefined.", subject)
}
