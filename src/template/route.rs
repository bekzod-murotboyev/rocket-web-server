use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/index")]
pub fn index() -> Template{
    Template::render("index",context! {field: "value"})
}