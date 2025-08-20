use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    _id: String,
    device: String,
    date: isize,
    #[serde(rename = "dateString")]
    date_string: String,
    sgv: f64,
    delta: f64,
    direction: String,
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

pub enum Unit {
    // Mgdl,
    Mmol,
}

impl Entry {
    pub fn get_measurement(&self, unit: Unit) -> f64 {
        match unit {
            // Unit::Mgdl => self.sgv,
            Unit::Mmol => Entry::mgdl_to_mmol(self.sgv),
        }
    }

    pub fn get_delta(&self, unit: Unit) -> f64 {
        match unit {
            // Unit::Mgdl => self.delta,
            Unit::Mmol => Entry::mgdl_to_mmol(self.delta),
        }
    }

    pub fn get_measured_at(&self) -> String {
        Entry::format_time(self.date_string.clone())
    }

    fn mgdl_to_mmol(mgdl: f64) -> f64 {
        mgdl / 18.0182
    }

    fn format_time(time: String) -> String {
        match time.parse::<DateTime<Local>>() {
            Ok(date_time) => date_time.format("%H:%M (%Y-%m-%d)").to_string(),
            Err(_) => time,
        }
    }
}
