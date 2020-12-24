use actix_web::{
    HttpResponse,
    web
};
use actix_cors::Cors;
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

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

pub fn get_secret_key() -> String {
    std::env::var("JWT_SECRET").unwrap_or("my-super-secret-key-for-elog-manager".into())
}

// Endpoints registration config
pub fn route_config(config: &mut web::ServiceConfig) {
    // Only imports of Endpoints
    use crate::handlers::{
        app_user_handler::{
            register,
            login
        },
        pay_type_handler::{
            insert_pay_type,
            get_all_pay_types
        },
        user_pay_method_handler::{
            insert_user_pay_method,
            get_all_user_pay_methods
        },
        expense_handler::{
            insert_expense,
            get_all_expenses
        }
    };

    config
        .service(register)
        .service(login)
        .service(insert_pay_type)
        .service(get_all_pay_types)
        .service(insert_user_pay_method)
        .service(get_all_user_pay_methods)
        .service(insert_expense)
        .service(get_all_expenses);
}
