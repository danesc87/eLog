use actix_web::{
    HttpResponse,
    web
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
pub type MySqlPool = Pool<ConnectionManager<MysqlConnection>>;
type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn connect() -> MySqlPool {
    init(DB_URL).expect("Error")
}

fn init(database_url: &str) -> Result<MySqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn mysql_pool_handler(pool: web::Data<MySqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// Endpoints registration config
pub fn route_config(config: &mut web::ServiceConfig) {
    // Only imports of Endpoints
    use crate::handlers::{
        sample::{
            hello_world,
            hello_user_name,
            test_sum
        },
        user_roles::{
            get_user_roles,
            get_user_role_with_id
        }
    };



    config.service(hello_world)
        .service(hello_user_name)
        .service(test_sum)
        .service(get_user_roles)
        .service(get_user_role_with_id);
}
