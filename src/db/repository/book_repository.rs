use crate::db::model::book_model::BookModel;
use crate::db::schema::book_schema::_book;
use crate::db::Db;
use rocket::serde::json::Json;
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

pub async fn find_one(mut db: Connection<Db>, id: i32) -> Option<Json<BookModel>> {
    _book::table
        .filter(_book::id.eq(id))
        .first(&mut db)
        .await
        .map(Json)
        .ok()
}

pub async fn find_all(mut db: Connection<Db>) -> Option<Json<Vec<BookModel>>> {
    _book::table
        .select(_book::all_columns)
        .load::<BookModel>(&mut db)
        .await
        .map(Json)
        .ok()
}