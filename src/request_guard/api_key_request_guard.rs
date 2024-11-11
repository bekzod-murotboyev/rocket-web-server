use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

const API_KEY_NAME: &str = "apiKey";
pub struct ApiKeyRequestGuard<'r>(pub &'r str);

#[derive(Debug)]
pub enum ApiKeyRequestGuardError {
    MISSING,
    INVALID,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyRequestGuard<'r> {
    type Error = ApiKeyRequestGuardError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let is_valid = |key: &str| key == "1";
        match request.query_fields().find(|param| param.name == API_KEY_NAME) {
            None => Outcome::Forward(Status::Unauthorized),
            Some(api_key) if is_valid(api_key.value) => Outcome::Success(ApiKeyRequestGuard(api_key.value)),
            Some(_) => Outcome::Error((Status::Unauthorized, ApiKeyRequestGuardError::INVALID))
        }
    }
}
