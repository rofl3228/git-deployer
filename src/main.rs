#[macro_use] extern crate rocket;
use rocket::serde::Deserialize;
use rocket::fairing::{AdHoc};

mod github;

#[cfg(git_deployer)] mod git_deployer; 

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    github_key: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(github::build_routes())
        .attach(AdHoc::config::<AppConfig>())
}
