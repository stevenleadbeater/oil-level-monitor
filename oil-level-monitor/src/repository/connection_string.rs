use std::env;

pub fn connection_string() -> String {
    "postgresql://".to_owned()
        + &env::var("DATABASE_USERNAME").unwrap().to_owned()
        + &":".to_owned()
        + &env::var("DATABASE_PASSWORD").unwrap().to_owned()
        + &"@".to_owned()
        + &env::var("DATABASE_HOST").unwrap().to_owned()
        + &":".to_owned()
        + &env::var("DATABASE_PORT").unwrap().to_owned()
        + &"/oil_level".to_owned()
}