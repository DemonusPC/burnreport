use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData {
    pub date: DateTime<Utc>,
    pub value: f64
}

impl TimeSeriesData {
    pub fn new(date: DateTime<Utc>, value: f64) -> Self {
        Self {
            date,
            value
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overview {
    mass: f64,
    fat: f64
}

impl Overview {
    pub fn new(mass: f64, fat: f64) -> Self {
        Self {
            mass,
            fat
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monthly {
    mass: Vec<TimeSeriesData>,
    fat: Vec<TimeSeriesData>
}

impl Monthly {
    pub fn new(mass: Vec<TimeSeriesData>, fat: Vec<TimeSeriesData>) -> Self {
        Self {
            mass,
            fat
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyOverview {
    today: Overview,
    monthly: Monthly 
}

impl BodyOverview {
    pub fn new(today: Overview, monthly: Monthly) -> Self {
        Self {
            today,
            monthly
        }
    }

    pub fn empty() -> Self {
        Self {
            today: Overview {
                mass: 0.0,
                fat: 0.0
            },
            monthly: Monthly {
                mass: vec![],
                fat: vec![]
            }
        }
    }

    pub fn today_mass(mut self, mass: f64) -> Self {
        self.today.mass = mass;
        self
    }

    pub fn today_fat(mut self, fat: f64) -> Self {
        self.today.fat = fat;
        self
    }

    pub fn add_mass_value(mut self, data: TimeSeriesData) -> Self {
        self.monthly.mass.push(data);
        self
    }

    pub fn add_fat_value(mut self, data: TimeSeriesData) -> Self {
        self.monthly.fat.push(data);
        self
    }
}