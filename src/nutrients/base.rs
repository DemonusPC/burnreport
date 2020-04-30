use serde_derive::{Deserialize, Serialize};

pub trait TotalAble {
    fn total(&self) -> f64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Energy {
    kcal: f64,
    kj: f64,
}

impl Energy {
    pub fn new(kcal: f64, kj: f64) -> Energy {
        Energy { kcal: kcal, kj: kj }
    }

    pub fn kcal(&self) -> f64 {
        return self.kcal;
    }

    pub fn k_j(&self) -> f64 {
        return self.kj;
    }
}

impl TotalAble for Energy {
    fn total(&self) -> f64 {
        return self.kcal;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Carbohydrates {
    total: f64,
    fiber: f64,
    sugar: f64,
    #[serde(rename = "addedSugar")]
    added_sugar: f64,
    starch: f64,
}

impl Carbohydrates {
    pub fn new(total: f64, fiber: f64, sugar: f64, added_sugar: f64, starch: f64) -> Carbohydrates {
        Carbohydrates {
            total: total,
            fiber: fiber,
            sugar: sugar,
            added_sugar: added_sugar,
            starch: starch,
        }
    }

    pub fn fiber(&self) -> f64 {
        return self.fiber;
    }
    pub fn sugar(&self) -> f64 {
        return self.sugar;
    }
    pub fn added_sugar(&self) -> f64 {
        return self.added_sugar;
    }
    pub fn starch(&self) -> f64 {
        return self.starch;
    }
    pub fn contains_added_sugar(&self) -> bool {
        if self.added_sugar > 0.0 {
            return true;
        }
        return false;
    }
}

impl TotalAble for Carbohydrates {
    fn total(&self) -> f64 {
        return self.total;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fat {
    total: f64,
    saturated: f64,
    monounsaturated: f64,
    trans: f64,
}

impl Fat {
    pub fn new(total: f64, saturated: f64, mono: f64, trans: f64) -> Fat {
        Fat {
            total: total,
            saturated: saturated,
            monounsaturated: mono,
            trans: trans,
        }
    }

    pub fn saturated(&self) -> f64 {
        return self.saturated;
    }

    pub fn monounsaturated(&self) -> f64 {
        return self.monounsaturated;
    }

    pub fn trans(&self) -> f64 {
        return self.trans;
    }

    pub fn contains_trans(&self) -> bool {
        if self.trans > 0.0 {
            return true;
        }
        return false;
    }
}

impl TotalAble for Fat {
    fn total(&self) -> f64 {
        return self.total;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Protein {
    total: f64,
}

impl Protein {
    pub fn new(total: f64) -> Protein {
        Protein { total: total }
    }
}

impl TotalAble for Protein {
    fn total(&self) -> f64 {
        return self.total;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Salt {
    total: f64,
}

impl Salt {
    pub fn new(total: f64) -> Salt {
        Salt { total: total }
    }
}

impl TotalAble for Salt {
    fn total(&self) -> f64 {
        return self.total;
    }
}
