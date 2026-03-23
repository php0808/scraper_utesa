mod config;
mod entities;
mod formatter;
mod parser;
mod scrapper;

use reqwest::Error;

use crate::{config::Config, entities::Response};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Config::from_env().unwrap();
    let scraper = scrapper::UtezaScrap::new();
    let params = &[
        ("campus", config.campus),
        ("username", config.username),
        ("password", config.password),
    ];

    scraper
        .login("https://nube.utesa.edu/local/entrada/", params)
        .await?;

    let sessions = scraper
        .extract_session("https://nube.utesa.edu/report/usersessions/user.php")
        .await?;

    let res = Response { sessions: sessions };
    let json = formatter::to_json(res);

    println!("{}", json);

    Ok(())
}
