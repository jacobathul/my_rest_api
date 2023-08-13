use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewUser {
    pub name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdatUser{
    pub name: Option<String>,
    pub email: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewBook {
    pub title: String,
    pub author: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>
}
