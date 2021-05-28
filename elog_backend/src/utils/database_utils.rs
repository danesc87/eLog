use actix_web::{
    web,
    HttpResponse
};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{
    ConnectionManager,
    Pool,
    PooledConnection
};

// DB Types.
// If some user wants to change MySQL with PostgresSQL or SQLite
// Should only change SqlConnection type and its import
pub type SqlConnection = MysqlConnection;
pub type SqlPool = Pool<ConnectionManager<SqlConnection>>;
pub type SqlPooledConnection = PooledConnection<ConnectionManager<SqlConnection>>;

pub fn connect_database() -> SqlPool {
    let elog_db_config = crate::ElogConfig::new().database;
    let manager = ConnectionManager::<SqlConnection>::new(elog_db_config.db_url);
    Pool::builder()
        .max_size(elog_db_config.pool_size)
        .build(manager)
        .expect("Error during connection to Data Base!")
}

pub fn pool_handler(
    connection_pool: Option<&web::Data<SqlPool>>
) -> Result<SqlPooledConnection, HttpResponse> {
    connection_pool
        .ok_or(HttpResponse::InternalServerError().json("No Data Base connection exists!".to_owned()))
        .and_then(|pool| {
             pool.get().map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
        })
}
