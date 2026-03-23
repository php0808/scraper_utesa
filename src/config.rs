use std::{env};

pub struct Config {
    pub username: String,
    pub password: String,
    pub campus: String,
}

impl Config {
    pub fn from_env() -> Result<Self, dotenvy::Error> {
        dotenvy::dotenv().ok();

        Ok(Self {
            username: env::var("USERNAME").unwrap(),
            password: env::var("PASSWORD").unwrap(),
            campus: env::var("CAMPUS").unwrap(),
        })
    }
}
