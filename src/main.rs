use rocket::launch;
use rocket::routes;

mod libs;
use libs::routes::hello;
use libs::routes::discords;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![hello::hello, discords::join_discord])
}