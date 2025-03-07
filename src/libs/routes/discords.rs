use rocket::put;
use rocket::serde::json::{json, Json, Value};
use crate::libs::controllers::discords_controller;
use crate::libs::utils::surreal_int::SurrealInt;
use serde::Deserialize;

#[derive(Deserialize)]
struct DiscordServerInput {
    discord_name: String,
    joined_at: String,
    balance: SurrealInt,
}

#[put("/discord/<discord_id>", data = "<discord_data>")]
pub async fn join_discord(discord_id: SurrealInt, discord_data: Json<DiscordServerInput>) -> String {
    let discord_name = discord_data.discord_name.clone();
    let joined_at = discord_data.joined_at.clone();
    let balance = discord_data.balance.clone();

    let result = discords_controller::join_discord(discord_id, discord_name, joined_at, balance).await;

    println!("result: {:?}", result);   

    match result {
        Ok(Some(_)) => "success".to_string(),
        Ok(None) => "error".to_string(),    
        Err(_) => "error".to_string(),
    }
}