// This file contains every mod, function, struct, etc
// that can be included on root level files, like config or main

pub mod elog {
    mod register;
    pub use self::register::hello_world;
    pub use self::register::hello_user_name;
    pub use self::register::test_sum;
}