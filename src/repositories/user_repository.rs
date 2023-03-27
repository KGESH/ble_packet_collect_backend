use crate::database::DB;
use crate::models::user::User;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;

pub async fn create(mut conn: Connection<DB>, user: User) {}
