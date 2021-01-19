use actix_web::{
    web,
    HttpResponse
};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{
    ConnectionManager,
    Pool,
    PoolError,
    PooledConnection
};

// DB Types.
// If some user wants to change MySQL with PostgresSQL or SQLite
// Should only change SqlConnection type and its import
pub type SqlConnection = MysqlConnection;
pub type SqlPool = Pool<ConnectionManager<SqlConnection>>;
pub type SqlPooledConnection = PooledConnection<ConnectionManager<SqlConnection>>;

pub fn connect_database() -> SqlPool {
    let db_url = super::env_variable_utils::get_variable("DB_URL");
    init_database(db_url.as_str()).expect("Error during connection to Data Base!")
}

fn init_database(database_url: &str) -> Result<SqlPool, PoolError> {
    let manager = ConnectionManager::<SqlConnection>::new(database_url);
    let pool_size = super::env_variable_utils::get_variable_as_integer("POOL_SIZE");
    Pool::builder().max_size(pool_size).build(manager)
}

pub fn pool_handler(connection_pool: Option<&web::Data<SqlPool>>) -> Result<SqlPooledConnection, HttpResponse> {
    connection_pool
        .ok_or(HttpResponse::InternalServerError().json("No Data Base connection exists!".to_owned()))
        .and_then(|pool| {
             pool.get().map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
        })
}
