mod nightscout;
use chrono::{DateTime, Local};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

use crate::nightscout::Nightscout;

#[derive(Debug, Serialize, Deserialize)]
struct KomlesukkerConfig {
    url: String,
    secret: String,
    threshold: Threshold,
}

#[derive(Debug, Serialize, Deserialize)]
struct Threshold {
    very_high: f64,
    high: f64,
    low: f64,
    very_low: f64,
}

// TODO:
// - [ ] Store some temporary state, to handle arguments passed in, like a "privacy-mode", which would have to keep track of on/off state.
// - [ ] Add "stale" class, with its own colour.
// - [ ] Click on the module could open the URL in a browser window?
// - [x] Format time since last update. At least readable, and in current time zone, but maybe "3 minutes ago".
// - [x] Add time since last update.
// - [x] Add +/- to the delta. - is already there, but I need to add + when it is positive.
// - [x] Move secret and URL to a config file or env variable.
// - [x] Figure out why the LSP is struggling so much! Watch the YT video I found.
// - [x] Figure out why Neoformat does not work with rustfmt.
//      - Didn't figure it out, but i changed it with native formatting, and now it works.

fn main() -> Result<(), Error> {
    let config = get_config();

    let nightscout = Nightscout::new(config.url.clone(), config.secret.clone());
    let latest = nightscout.get_latest()?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match latest {
        Some(entry) => {
            let sgv = mgdl_to_mmol(entry.sgv);
            let delta = format_delta(mgdl_to_mmol(entry.delta));

            let text = format!("{sgv:.1} ({delta})");
            let time = format_time(entry.date_string.clone());
            let class = get_class(sgv, config);

            let out = format!(
                "{{ \"text\": \"{text}\", \"tooltip\": \"{time}\", \"class\": \"{class}\" }}"
            );

            handle
                .write_all(out.as_bytes())
                .expect("Could not write to stdout");
        }
        None => {
            handle
                .write_all(b"{{ \"text\": \"No entry found\", \"class\": \"error\" }}")
                .expect("Could not write to stdout");
        }
    };

    return Ok(());
}

fn get_config() -> KomlesukkerConfig {
    let os_config_dir = dirs::config_dir().expect("Could not get the config directory");
    let app_config_dir = os_config_dir.join("komlesukker/config.json");
    let config = fs::read_to_string(app_config_dir).expect("Could not find the config file. Add a config file to \"~/.config/komlesukker/config.json\"");
    return serde_json::from_str(config.as_str()).expect("Could not parse JSON config");
}

fn format_delta(delta: f64) -> String {
    if delta >= 0.0 {
        format!("+{delta:.1}")
    } else {
        format!("{delta:.1}")
    }
}

fn format_time(time: String) -> String {
    match time.parse::<DateTime<Local>>() {
        Ok(date_time) => date_time.format("%H:%M (%Y-%m-%d)").to_string(),
        Err(_) => time,
    }
}

fn get_class(sgv: f64, config: KomlesukkerConfig) -> String {
    if sgv >= config.threshold.very_high {
        String::from("very-high")
    } else if sgv >= config.threshold.high {
        String::from("high")
    } else if sgv <= config.threshold.low {
        String::from("low")
    } else if sgv <= config.threshold.very_low {
        String::from("very-low")
    } else {
        String::from("normal")
    }
}

fn mgdl_to_mmol(mgdl: f64) -> f64 {
    return mgdl / 18.0182;
}
