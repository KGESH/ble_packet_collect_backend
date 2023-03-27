use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("example_db")]
pub struct DB(sqlx::PgPool);
