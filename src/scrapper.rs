use crate::{
    entities::{ Session},
    parser,
};
use reqwest::{Client, Error};

pub struct UtezaScrap {
    pub client: Client,
}

impl UtezaScrap {
    pub fn new() -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();

        Self { client: client }
    }

     async  fn fetch(&self, url: &str) -> Result<String, Error> {
        let res = self.client.get(url).send().await?.error_for_status()?;
        Ok(res.text().await?)
    }

    pub async fn login(&self, url: &str, params: &[(&str, String); 3]) -> Result<(), Error> {
        
        self.client
            .post(url)
            .form(params)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn extract_session(&self,session_url: &str) -> Result<Vec<Session> , Error> {
        
        let html = self.fetch(session_url).await?;
   
        let access_date = parser::extract_access_date(&html).unwrap_or_default();
        let last_access = parser::extract_last_access(&html).unwrap_or_default();
        let ip = parser::extract_ip(&html).unwrap_or_default();

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
    
        Ok(sessions)
    }
}
