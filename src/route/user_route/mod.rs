use rocket::fairing::AdHoc;
use rocket::routes;

mod route;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("USER ROUTE", |rocket| async {
        rocket
            .mount("/api/user", routes![
                route::save_user
            ])
    })
}