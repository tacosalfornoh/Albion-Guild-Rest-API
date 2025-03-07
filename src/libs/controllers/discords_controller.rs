use crate::libs::db::connect::connect_db;
use crate::libs::models::discords::DiscordServer;
use crate::libs::utils::surreal_int::SurrealInt;

// Funzione per connettersi al DB e selezionare i dati
pub async fn get_discords() -> Result<Vec<DiscordServer>, Box<dyn std::error::Error>> {
    let db = connect_db().await?;

    let table = db.select("discords").await?;

    Ok(table)
}

pub async fn join_discord(
    discord_id: SurrealInt,
    discord_name: String,
    joined_at: String,
    balance: SurrealInt,
) -> Result<Option<DiscordServer>, Box<dyn std::error::Error>> {
    let db = connect_db().await?;

    // Check if the discord_id already exists
    let existing_discord: Option<DiscordServer> = db.select(("discords", discord_id.to_string())).await?;

    match existing_discord {
        Some(mut existing) => {
            // Discord ID exists, update only the discord_name
            existing.discord_name = Some(discord_name);
            let updated_discord: Option<DiscordServer> = db.update(("discords", discord_id.to_string())).content(existing).await?;
            Ok(updated_discord)
        }
        None => {
            // Discord ID does not exist, create a new record
            let discord = DiscordServer {
                discord_id: Some(discord_id),
                discord_name: Some(discord_name),
                joined_at: Some(joined_at),
                balance: Some(balance),
            };

            let created_discord: Option<DiscordServer> = db.create(("discords", discord_id.to_string()))
                .content(discord)
                .await?;
            Ok(created_discord)
        }
    }
}
