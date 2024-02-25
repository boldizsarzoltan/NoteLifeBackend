use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn get_connection() -> PgConnection {

    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let db_name = env::var("POSTGRES_DATABASE").expect("POSTGRES_USER not set");

    // Compose the database URL
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    println!("{}", database_url);

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}