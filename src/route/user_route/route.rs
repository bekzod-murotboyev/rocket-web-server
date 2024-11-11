use crate::data_guard::user_data_guard::UserDataGuard;
use rocket::post;
use rocket::serde::json::Json;

#[post("/", data = "<user>")]
pub fn save_user(user: Json<UserDataGuard<'_>>) -> Json<UserDataGuard> {
    if let Some(name) = user.name {
        println!("User's name is {name}")
    }
    user
}