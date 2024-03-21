use super::schema::users;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) message: String,
}
