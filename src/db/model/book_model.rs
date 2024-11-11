use crate::db::schema::book_schema::_book;
use rocket_db_pools::diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = _book)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BookModel {
    pub id: i32,
    pub title: String,
}