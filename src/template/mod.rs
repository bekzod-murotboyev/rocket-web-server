mod route;

use rocket::fairing::AdHoc;
use rocket::routes;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("TEMPLATE", |rocket| async {
        rocket
            .mount("/", routes![route::index])
    })
}