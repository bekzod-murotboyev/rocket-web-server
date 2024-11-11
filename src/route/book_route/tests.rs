use rocket::http::Status;
use rocket::local::blocking::Client;

use crate::rocket;

#[test]
fn find_all_test() {
    let client = Client::tracked(rocket()).unwrap();
    let mut response = client.get("/api/book").dispatch();
    assert_eq!(response.status(), Status::Ok);
}