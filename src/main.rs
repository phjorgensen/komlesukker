mod komle_config;
mod nightscout;

use crate::{
    komle_config::KomlesukkerConfig,
    nightscout::{Nightscout, entry::Unit},
};
use reqwest::Error;
use std::io::{self, Write};

// TODO:
// - [ ] Store some temporary state, to handle arguments passed in, like a "privacy-mode", which would have to keep track of on/off state.
// - [ ] Add "stale" class, with its own colour.
// - [ ] Click on the module could open the URL in a browser window?

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = KomlesukkerConfig::new();
    let nightscout = Nightscout::new(config.get_url(), config.get_secret());
    let latest = nightscout.get_latest().await?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match latest {
        Some(entry) => {
            let measurement = entry.get_measurement(Unit::Mmol);
            let delta = entry.get_delta(Unit::Mmol);
            let delta_prefix = if delta >= 0.0 { "+" } else { "" };
            let measured_at = entry.get_measured_at();

            let text = format!("{measurement:.1} ({delta_prefix}{delta:.1})");
            let class = get_class(measurement, config);

            let out = format!(
                "{{ \"text\": \"{text}\", \"tooltip\": \"{measured_at}\", \"class\": \"{class}\" }}"
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

fn get_class(sgv: f64, config: KomlesukkerConfig) -> String {
    let thresholds = config.get_thresholds();

    if sgv >= thresholds.very_high {
        String::from("very-high")
    } else if sgv >= thresholds.high {
        String::from("high")
    } else if sgv <= thresholds.very_low {
        String::from("very-low")
    } else if sgv <= thresholds.low {
        String::from("low")
    } else {
        String::from("normal")
    }
}
