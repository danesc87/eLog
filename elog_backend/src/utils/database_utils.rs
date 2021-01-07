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

// TODO This should be build based on "env" variables
const DB_URL: &'static str = "mysql://root:1234abcd@127.0.0.1:3306/elog";

// DB Config
pub type SqlConnection = MysqlConnection;
pub type SqlPool = Pool<ConnectionManager<SqlConnection>>;
pub type SqlPooledConnection = PooledConnection<ConnectionManager<SqlConnection>>;

pub fn connect_database() -> SqlPool {
    init_database(DB_URL).expect("Error")
}

fn init_database(database_url: &str) -> Result<SqlPool, PoolError> {
    let manager = ConnectionManager::<SqlConnection>::new(database_url);
    Pool::builder().max_size(6).build(manager)
}

pub fn pool_handler(connection_pool: Option<&web::Data<SqlPool>>) -> Result<SqlPooledConnection, HttpResponse> {
    connection_pool
        .ok_or(HttpResponse::InternalServerError().json("No DB connection exists".to_owned()))
        .and_then(|c| {
             c.get().map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
        })
}
