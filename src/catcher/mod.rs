mod catcher;

use rocket::fairing::AdHoc;
use rocket::catchers;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("CATCHER", |rocket| async {
        rocket
            .register("/", catchers![
                catcher::catch_default,
                catcher::catch_not_found_exception
        ])
    })
}