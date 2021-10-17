use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSubmission {
    id: i32,
    name: String,
    amount: f64,
}

impl ProductSubmission {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub consumed: Vec<ProductSubmission>,
}
