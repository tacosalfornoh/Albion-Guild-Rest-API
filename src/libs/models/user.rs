use faker_rand::en_us::internet::Username;
use serde::{Deserialize, Serialize};
use surrealdb::Datetime;
use surrealdb::sql::{Number, Thing};
use surrealdb::syn::lexer::compound::Numeric;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub server_name: String,
    pub discord_id: Number,
    pub joined_date: Datetime,
    pub ign_link: String,
    pub balance: String,
    pub vods: Number,
    pub attendance: Number
}