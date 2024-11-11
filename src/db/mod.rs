use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::Database;

pub mod model;
pub mod schema;
pub mod repository;

#[derive(Database)]
#[database("rocket_db")]
pub struct Db(PgPool);