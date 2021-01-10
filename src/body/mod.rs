use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyLog {
    date: DateTime<Utc>,
    mass: f64,
    fat: f64,
}

impl BodyLog {
    pub fn new(date: DateTime<Utc>, mass: f64, fat: f64) -> Self {
        Self { date, mass, fat }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn fat(&self) -> f64 {
        self.fat
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData {
    pub date: DateTime<Utc>,
    pub value: f64,
}

impl TimeSeriesData {
    pub fn new(date: DateTime<Utc>, value: f64) -> Self {
        Self { date, value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overview {
    mass: f64,
    fat: f64,
}

impl Overview {
    pub fn new(mass: f64, fat: f64) -> Self {
        Self { mass, fat }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyOverview {
    today: Option<Overview>,
    past: Vec<BodyLog>,
}


impl BodyOverview {
    pub fn new(today: Option<Overview>, past: Vec<BodyLog>) -> Self {
        Self { today, past }
    }

    pub fn new_from_log(sorted_past: Vec<BodyLog>) -> Self {
        let today_date = chrono::Utc::today().and_hms(0, 0, 0);
        // We assume that the log is already sorted by date
        let today = match sorted_past.first() {
            Some(log_val) => match log_val.date().date() == today_date.date() {
                true => Some(Overview::new(log_val.mass(), log_val.fat())),
                false => None,
            },
            None => None,
        };

        Self {
            today,
            past: sorted_past,
        }
    }
}
