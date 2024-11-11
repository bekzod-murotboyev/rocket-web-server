use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::db::Db;
use crate::db::model::book_model::BookModel;
use crate::db::repository::book_repository;
use crate::request_guard::api_key_request_guard::ApiKeyRequestGuard;

#[get("/<id>")]
pub async fn find_by_id(db: Connection<Db>, id: i32) -> Option<Json<BookModel>> {
    book_repository::find_one(db, id).await
}

#[get("/")]
pub async fn find_all(db: Connection<Db>) -> Option<Json<Vec<BookModel>>> {
    book_repository::find_all(db).await
}

#[get("/<name>", rank = 1)]
pub fn get_book_name(name: &str) -> String {
    format!("Name of the book is, {}!", name)
}

#[get("/secure")]
pub fn request_secure(api_key: ApiKeyRequestGuard) -> String {
    format!("Secure endpoint: {}", api_key.0)
}
#[get("/secure", rank = 0)]
pub fn request_unsecure() -> &'static str {
    "Not secure endpoint"
}