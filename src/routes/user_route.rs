use crate::database::DB;
use crate::models::user::User;
use crate::services::user_service;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_db_pools::sqlx::postgres::PgQueryResult;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/<id>")]
pub async fn get_user(mut conn: Connection<DB>, id: i32) -> Result<Json<User>> {
    let user = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await
        .and_then(|row| {
            Ok(User {
                id: row.try_get(0)?,
                age: row.try_get(1)?,
                name: row.try_get(2)?,
            })
        })
        .ok();

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(rocket::response::Debug(sqlx::Error::RowNotFound)),
    }
}

#[post("/", data = "<user>")]
pub async fn create_user(
    mut conn: Connection<DB>,
    user: Json<User>,
) -> Result<Created<Json<User>>> {
    let created_user =
        sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id, age, name")
            .bind(&user.name)
            .bind(&user.age)
            .fetch_one(&mut *conn)
            .await
            .and_then(|row| {
                Ok(User {
                    id: row.try_get(0)?,
                    age: row.try_get(1)?,
                    name: row.try_get(2)?,
                })
            });

    match created_user {
        Ok(user) => Ok(Created::new("/").body(Json(user))),
        Err(e) => Err(rocket::response::Debug(e)),
    }
}
