use rocket::serde::json::{Json, Value};
use rocket::get;
use crate::libs::controllers::hello_controller;

#[get("/")]
pub async fn hello() -> Json<Value> {
    hello_controller::get_users().await
}