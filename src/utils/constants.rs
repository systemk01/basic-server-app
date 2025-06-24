use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref Database_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string())
}
fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080)
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("Postgres DATABASE_URL for basic_server_db not set")
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap_or("SECRET".to_string())
}