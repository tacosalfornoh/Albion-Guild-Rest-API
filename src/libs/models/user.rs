use serde::{Deserialize, Serialize};
use surrealdb::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Number,
    pub username: String,
    pub server_name: String,
    pub discord_id: Number,
    pub joined_date: String,
    pub ign_link: String,
    pub balance: String,
    pub vods: Number,
    pub attendance: Number
}