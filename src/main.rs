extern crate rocket;
use rocket::{launch, Build, Rocket};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use std::sync::atomic::AtomicI32;

mod request_guard;
mod data_guard;
mod route;
mod catcher;
mod template;
mod db;
mod fairing;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(route::init())
        .attach(template::init())
        .attach(catcher::init())
        .attach(db::Db::init())
        .attach(Template::fairing())
        .attach(fairing::request_log_fairing::RequestLog { count: AtomicI32::new(0) })
}