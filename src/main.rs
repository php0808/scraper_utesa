mod entities;
mod formatter;
mod parser;
mod scrapper;

use crate::entities::Response;
use crate::entities::Session;
use crate::scrapper::UtezaScrap;
use dotenvy::{self, dotenv};
use reqwest::{Client, Error};
#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let campus = std::env::var("CAMPUS").unwrap_or("".to_string());
    let matricula = std::env::var("MATRICULA").unwrap_or("".to_string());
    let pass = std::env::var("PASS").unwrap_or("".to_string()); 
    
    
    let client = Client::builder().cookie_store(true).build()?;
    let params = [
        ("campus", campus),
        ("username", matricula),
        ("password", pass),
    ];
    let login_url = "https://nube.utesa.edu/local/entrada/";
    let session_url = "https://nube.utesa.edu/report/usersessions/user.php";

    let scrap = UtezaScrap { client: client };
    scrap.login(login_url, &params).await?;
    let html = scrap.fetch(session_url).await?;

    let access_date = parser::extract_access_date(&html).unwrap();
    let last_access = parser::extract_last_access(&html).unwrap();

    let ip = parser::extract_ip(&html).unwrap();

    let sessions: Vec<Session> = ip
        .into_iter()
        .zip(access_date)
        .zip(last_access)
        .map(|((ip, access_date), last_access)| Session {
            ip: ip,
            access: access_date,
            last_access: last_access,
        })
        .collect();

    let re = Response { sessions: sessions };
    let res_json = formatter::to_json(re);

    print!("{}", res_json);
    Ok(())
}
