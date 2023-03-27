use crate::database::DB;
use crate::models::user::User;
use crate::repositories::user_repository;
use rocket_db_pools::Connection;

pub async fn create_user(mut conn: Connection<DB>) {
    let user = User {
        id: 0,
        age: 26,
        name: "jee".to_string(),
    };

    user_repository::create(conn, user).await;
}
