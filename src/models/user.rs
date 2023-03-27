use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub age: i32,
    pub name: String,
}
