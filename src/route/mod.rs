use rocket::fairing::AdHoc;

pub mod book_route;
mod user_route;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("ROUTE", |rocket| async {
        rocket
            .attach(user_route::init())
            .attach(book_route::init())
    })
}
