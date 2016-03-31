use std::env;

pub fn get_default_user() -> String {
    env::var("USER").unwrap()
}
