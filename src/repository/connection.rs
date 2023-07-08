use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn get_connection() -> PgConnection {
    dotenv().ok();  

    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    println!("{}", db_host);
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
    println!("{}", db_port);
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    println!("{}", db_user);
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    println!("{}", db_password);
    let db_name = String::from("note_life");
    println!("{}", db_name);

    // Compose the database URL
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    println!("{}", database_url);

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}