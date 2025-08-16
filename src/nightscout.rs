use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

pub struct Nightscout {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    _id: String,
    device: String,
    date: isize,
    #[serde(rename = "dateString")]
    pub date_string: String,
    pub sgv: f64,
    pub delta: f64,
    pub direction: String,
    #[serde(rename = "type")]
    entry_type: String,
    filtered: i32,
    unfiltered: i32,
    rssi: i32,
    noise: i32,
    #[serde(rename = "sysTime")]
    sys_time: String,
    #[serde(rename = "utcOffset")]
    utc_offset: i32,
    mills: isize,
}

impl Nightscout {
    pub fn new(url: String, secret: String) -> Nightscout {
        Nightscout {
            client: Client::new(),
            base_url: format!("https://{}@{}/api/v1", secret, url),
        }
    }

    #[tokio::main]
    pub async fn get_latest(&self) -> Result<Option<Entry>, Error> {
        let url = format!("{}/entries/sgv?count=1", self.base_url,);

        let entries: Vec<Entry> = self
            .client
            .get(url)
            .header("accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        return match entries.get(0) {
            Some(entry) => Ok(Some(entry.clone())),
            None => Ok(None),
        };
    }
}
