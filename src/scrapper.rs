use reqwest::{Client, Error};

pub struct UtezaScrap {
    pub client: Client,
}

impl UtezaScrap {



    pub async fn fetch(&self, url: &str) -> Result<String, Error> {
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
}
