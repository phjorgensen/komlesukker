mod komle_config;
mod nightscout;

use crate::{
    komle_config::KomlesukkerConfig,
    nightscout::{Nightscout, entry::Unit},
};
use clap::Parser;
use reqwest::Error;
use std::io::{self, Write};

// TODO:
// Store some temporary state, to handle arguments passed in, like a "privacy-mode", which would have to keep track of on/off state.
// Add "stale" class, with its own colour.
// Create a file that stores some info, like last measurement, settings, like privcy.
// Add arrow icons.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Privacy mode
    #[arg(short, long, default_value_t = false)]
    privacy: bool,

    /// Open the URL in a browser window
    #[arg(short, long, default_value_t = false)]
    open_in_browser: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let stdout = io::stdout();
    let config = KomlesukkerConfig::new();

    if args.open_in_browser {
        webbrowser::open(&format!("https://{}", config.get_url()))
            .expect("Could not open the browser.");

        return Ok(());
    }

    if args.privacy {
        let out = "{ \"text\": \"-.- (-.-)\", \"tooltip\": \"Privacy mode is turned on\", \"class\": \"privacy\" }";

        stdout
            .lock()
            .write_all(out.as_bytes())
            .expect("Could not write to stdout");

        return Ok(());
    }

    let nightscout = Nightscout::new(config.get_url(), config.get_secret());
    let latest = nightscout.get_latest().await?;

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

            stdout
                .lock()
                .write_all(out.as_bytes())
                .expect("Could not write to stdout");
        }
        None => {
            stdout
                .lock()
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
