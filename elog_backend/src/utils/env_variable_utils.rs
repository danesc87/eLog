use std::collections::HashMap;

// Default variable values
// If some variable doesn't exist on ".env" file will get it from this Map
lazy_static! {
    static ref DEFAULT_VARIABLES: HashMap<&'static str, &'static str> =
        [
            ("SERVER_IP","0.0.0.0"),
            ("SERVER_PORT", "8090"),
            ("RUST_LOG", "actix_web=debug"),
            ("DB_URL", "mysql://root:1234abcd@127.0.0.1:3306/elog"),
            ("POOL_SIZE", "6"),
            ("JWT_SECRET", "elog-super-secret-key"),
            ("TOKEN_DURATION_MIN", "60"),
        ].iter().cloned().collect();
}

pub fn get_variable(var_name: &str) -> String {
    std::env::var(var_name)
        .unwrap_or(
            DEFAULT_VARIABLES
                .get(var_name)
                .unwrap()
                .to_owned()
                .to_string()
        )
}

pub fn get_variable_as_integer(var_name: &str) -> u32 {
    std::env::var(var_name)
        .unwrap_or(
            DEFAULT_VARIABLES
                .get(var_name)
                .unwrap()
                .to_owned()
                .to_string()
        ).parse::<u32>().unwrap()
}
