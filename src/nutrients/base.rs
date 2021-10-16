use serde_derive::{Deserialize, Serialize};
use std::ops::Add;

use super::{add_options, Vitamins};

pub trait TotalAble {
    fn total(&self) -> f64;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Energy {
    kcal: f64,
    kj: f64,
}

impl Energy {
    pub fn new(kcal: f64, kj: f64) -> Energy {
        Energy { kcal, kj }
    }

    pub fn kcal(&self) -> f64 {
        self.kcal
    }

    pub fn k_j(&self) -> f64 {
        self.kj
    }
}

impl TotalAble for Energy {
    fn total(&self) -> f64 {
        self.kcal
    }
}

impl Add for Energy {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            kj: self.kj + other.kj,
            kcal: self.kcal + other.kcal,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Carbohydrates {
    total: f64,
    sugar: f64,
    fiber: Option<f64>,
    #[serde(rename = "addedSugar")]
    added_sugar: Option<f64>,
    starch: Option<f64>,
}

impl Carbohydrates {
    pub fn new(
        total: f64,
        sugar: f64,
        fiber: Option<f64>,
        added_sugar: Option<f64>,
        starch: Option<f64>,
    ) -> Carbohydrates {
        Carbohydrates {
            total,
            sugar,
            fiber,
            added_sugar,
            starch,
        }
    }
    pub fn sugar(&self) -> f64 {
        self.sugar
    }

    pub fn fiber(&self) -> Option<f64> {
        self.fiber
    }
    pub fn added_sugar(&self) -> Option<f64> {
        self.added_sugar
    }
    pub fn starch(&self) -> Option<f64> {
        self.starch
    }
}

impl TotalAble for Carbohydrates {
    fn total(&self) -> f64 {
        self.total
    }
}

impl Add for Carbohydrates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            sugar: self.sugar + other.total,
            fiber: add_options(&self.fiber, &other.fiber),
            added_sugar: add_options(&self.added_sugar, &other.added_sugar),
            starch: add_options(&self.starch, &other.starch),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Protein {
    total: f64,
}

impl Protein {
    pub fn new(total: f64) -> Protein {
        Protein { total }
    }
}

impl TotalAble for Protein {
    fn total(&self) -> f64 {
        self.total
    }
}

impl Add for Protein {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Salt {
    total: f64,
}

impl Salt {
    pub fn new(total: f64) -> Salt {
        Salt { total }
    }
}

impl TotalAble for Salt {
    fn total(&self) -> f64 {
        self.total
    }
}

impl Add for Salt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Nutrients {
    energy: Energy,
    carbohydrates: Carbohydrates,
    fat: Fat,
    protein: Protein,
    salt: Salt,
    #[serde(skip_serializing_if = "Option::is_none")]
    vitamins: Option<Vitamins>,
}

impl Nutrients {
    pub fn new(
        energy: Energy,
        carbohydrates: Carbohydrates,
        fat: Fat,
        protein: Protein,
        salt: Salt,
        vitamins: Option<Vitamins>,
    ) -> Self {
        Self {
            energy,
            carbohydrates,
            fat,
            protein,
            salt,
            vitamins,
        }
    }

    pub fn energy(&self) -> &Energy {
        &self.energy
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        &self.carbohydrates
    }

    pub fn fat(&self) -> &Fat {
        &self.fat
    }

    pub fn protein(&self) -> &Protein {
        &self.protein
    }

    pub fn salt(&self) -> &Salt {
        &self.salt
    }

    pub fn vitamins(&self) -> Option<Vitamins> {
        self.vitamins.clone()
    }
}

impl Add for Nutrients {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            energy: self.energy + other.energy,
            carbohydrates: self.carbohydrates + other.carbohydrates,
            fat: self.fat + other.fat,
            protein: self.protein + other.protein,
            salt: self.salt + other.salt,
            vitamins: add_options(&self.vitamins, &other.vitamins),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Fat {
    total: f64,
    saturated: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    unsaturated: Option<UnsaturatedFat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trans: Option<f64>,
}

impl Fat {
    pub fn new(
        total: f64,
        saturated: f64,
        unsaturated: Option<UnsaturatedFat>,
        trans: Option<f64>,
    ) -> Self {
        Self {
            total,
            saturated,
            unsaturated,
            trans,
        }
    }

    pub fn saturated(&self) -> f64 {
        self.saturated
    }

    pub fn unsaturated(&self) -> Option<UnsaturatedFat> {
        self.unsaturated
    }

    pub fn trans(&self) -> Option<f64> {
        self.trans
    }
}

impl TotalAble for Fat {
    fn total(&self) -> f64 {
        self.total
    }
}

impl Add for Fat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            saturated: self.saturated + other.saturated,
            unsaturated: add_options(&self.unsaturated, &other.unsaturated),
            trans: add_options(&self.trans, &other.trans),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnsaturatedFat {
    #[serde(skip_serializing_if = "Option::is_none")]
    mono: Option<MonoUnsaturatedFat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    poly: Option<PolyUnsaturatedFat>,
}

impl UnsaturatedFat {
    pub fn new(mono: Option<MonoUnsaturatedFat>, poly: Option<PolyUnsaturatedFat>) -> Self {
        Self { mono, poly }
    }

    pub fn mono(&self) -> Option<MonoUnsaturatedFat> {
        self.mono
    }
    pub fn poly(&self) -> Option<PolyUnsaturatedFat> {
        self.poly
    }
}

impl Add for UnsaturatedFat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            mono: add_options(&self.mono, &other.mono),
            poly: add_options(&self.poly, &other.poly),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MonoUnsaturatedFat {
    total: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    omega_7: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omega_9: Option<f64>,
}

impl MonoUnsaturatedFat {
    pub fn new(total: f64, omega_7: Option<f64>, omega_9: Option<f64>) -> Self {
        Self {
            total,
            omega_7,
            omega_9,
        }
    }
    pub fn omega_7(&self) -> Option<f64> {
        self.omega_7
    }
    pub fn omega_9(&self) -> Option<f64> {
        self.omega_9
    }
}

impl TotalAble for MonoUnsaturatedFat {
    fn total(&self) -> f64 {
        self.total
    }
}

impl Add for MonoUnsaturatedFat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            omega_7: add_options(&self.omega_7, &other.omega_7),
            omega_9: add_options(&self.omega_9, &other.omega_9),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PolyUnsaturatedFat {
    total: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    omega_3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omega_6: Option<f64>,
}

impl PolyUnsaturatedFat {
    pub fn new(total: f64, omega_3: Option<f64>, omega_6: Option<f64>) -> Self {
        Self {
            total,
            omega_3,
            omega_6,
        }
    }
    pub fn omega_3(&self) -> Option<f64> {
        self.omega_3
    }

    pub fn omega_6(&self) -> Option<f64> {
        self.omega_6
    }
}

impl Add for PolyUnsaturatedFat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            omega_3: add_options(&self.omega_3, &other.omega_3),
            omega_6: add_options(&self.omega_6, &other.omega_6),
        }
    }
}

impl TotalAble for PolyUnsaturatedFat {
    fn total(&self) -> f64 {
        self.total
    }
}
