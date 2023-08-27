use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use crate::repository::connection::get_connection;
use diesel_migrations::embed_migrations;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations() {
    let mut conn = get_connection();
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}
