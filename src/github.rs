use std::fmt::Debug;

use rocket::http::Status;
use rocket::{State};
use rocket::fairing::AdHoc;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::{status};

use crate::AppConfig;

pub fn build_routes() -> AdHoc {
    AdHoc::on_ignite("Github deploying hook", |rocket| async {
        rocket.mount("/github", routes![index])
    })
}

#[derive(Debug, Clone)]
struct ApiKey(String);

impl ApiKey {
    fn to_string(&self) -> String {
        String::from(self.0.clone())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, Self::Error> {
        request
            .headers()
            .get_one("X-Hub-Signature-256")
            .map(|header| ApiKey(String::from(header)))
            .or_forward(())
    }
}

#[post("/", format = "application/json")]
fn index(api_key: ApiKey, app_config: &State<AppConfig>) -> status::Custom<&str> {
    println!("api key: {:?}, {:?}", api_key, app_config);
    if !api_key.to_string().eq(&app_config.github_key) {
        return status::Custom(Status::Forbidden, "Invalid API key");
    }

    status::Custom(Status::Accepted, "Accepted")
}
