use rocket::fairing::AdHoc;
use rocket::routes;

#[cfg(test)]
mod tests;
mod route;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("BOOK ROUTE", |rocket| async {
        rocket

            .mount("/api/book", routes![
                route::find_all,
                route::find_by_id,
                route::get_book_name,
                route::request_secure,
                route::request_unsecure
            ])
    })
}