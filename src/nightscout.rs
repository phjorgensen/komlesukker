pub mod entry;

use crate::nightscout::entry::Entry;
use reqwest::{Client, Error};

pub struct Nightscout {
    client: Client,
    base_url: String,
}

impl Nightscout {
    pub fn new(url: String, secret: String) -> Nightscout {
        Nightscout {
            client: Client::new(),
            base_url: format!("https://{}@{}/api/v1", secret, url),
        }
    }

    pub async fn get_entries(&self, count: u32) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/entries/sgv?count={count}", self.base_url,);

        let entries: Vec<Entry> = self
            .client
            .get(url)
            .header("accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        return Ok(entries);
    }

    pub async fn get_latest(&self) -> Result<Option<Entry>, Error> {
        match Nightscout::get_entries(&self, 1).await?.get(0) {
            Some(entry) => Ok(Some(entry.clone())),
            None => Ok(None),
        }
    }
}
