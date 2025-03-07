use rocket::serde::json::{json, Json, Value};

use crate::libs::db::connect::connect_db;
use crate::libs::models::user::User;

// Funzione per connettersi al DB e selezionare i dati
async fn get_users_from_db() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let db = connect_db().await?;

    // Insert a new users with name 'Johana' if not exists
    let result = db
        .query("SELECT * FROM users WHERE user_id = '12334'")
        .await
        .map_err(|e| {
            eprintln!("Failed to query users: {}", e);
            e
        })?;

    // Query all Users
    let mut result = db.query("SELECT user_id FROM users").await.map_err(|e| {
        eprintln!("Failed to query users: {}", e);
        e
    })?;
    println!("Query result: {:?}", result);

    let users: Vec<User> = result.take(0).map_err(|e| {
        eprintln!("Failed to take result: {}", e);
        e
    })?;

    println!("People found: {:?}", users);

    Ok(users)
}

pub async fn get_users() -> Json<Value> {
    match get_users_from_db().await {
        Ok(users) => {
            if users.is_empty() {
                Json(json!({ "message": "No users found" }))
            } else {
                Json(json!(users))
            }
        }
        Err(e) => {
            eprintln!("Errore durante l'accesso al db: {}", e);
            Json(json!({ "error": "Error accessing the database", "details": e.to_string() }))
        }
    }
}
