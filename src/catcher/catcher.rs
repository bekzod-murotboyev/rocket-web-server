use rocket::{catch, Request};
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

#[catch(default)]
pub fn catch_default(status: Status, _req: &Request) -> String {
    format!("Server responded with status code of: {}", status.code)
}

#[catch(404)]
pub fn catch_not_found_exception(_status: Status, _req: &Request) -> Template {
    Template::render("404", context! {})
}