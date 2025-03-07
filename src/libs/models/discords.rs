use serde::{Deserialize, Serialize};
use crate::libs::utils::surreal_int::SurrealInt;

// Definisce una struttura chiamata Discord che contiene informazioni su un utente Discord.
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordServer {
    pub discord_id: Option<SurrealInt>, // ID dell'utente Discord (utilizza la struttura SurrealInt).
    pub discord_name: Option<String>, // Nome del server.
    pub joined_at: Option<String>, // Data di ingresso nel server.
    pub balance: Option<SurrealInt>, // Saldo dell'utente (utilizza la struttura SurrealInt).
}