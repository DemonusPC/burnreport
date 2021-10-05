use serde_derive::{Deserialize, Serialize};

use super::Vitamins;

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
        return self.sugar;
    }

    pub fn fiber(&self) -> Option<f64> {
        return self.fiber.clone();
    }
    pub fn added_sugar(&self) -> Option<f64> {
        return self.added_sugar.clone();
    }
    pub fn starch(&self) -> Option<f64> {
        return self.starch.clone();
    }
    pub fn contains_added_sugar(&self) -> bool {
        self.added_sugar().is_some()
    }
}

impl TotalAble for Carbohydrates {
    fn total(&self) -> f64 {
        return self.total;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolyunsaturatedFats {
    omega_3: f64,
    omega_6: f64,
}

impl PolyunsaturatedFats {
    pub fn omega_3(&self) -> f64 {
        self.omega_3
    }

    pub fn omega_6(&self) -> f64 {
        self.omega_6
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fat {
    total: f64,
    saturated: f64,
    monounsaturated: f64,
    trans: f64,
    polyunsaturated: PolyunsaturatedFats,
}

impl Fat {
    pub fn new(
        total: f64,
        saturated: f64,
        mono: f64,
        trans: f64,
        omega_3: f64,
        omega_6: f64,
    ) -> Fat {
        Fat {
            total: total,
            saturated: saturated,
            monounsaturated: mono,
            trans: trans,
            polyunsaturated: PolyunsaturatedFats { omega_3, omega_6 },
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

    pub fn omega_3(&self) -> f64 {
        return self.polyunsaturated.omega_3();
    }

    pub fn omega_6(&self) -> f64 {
        return self.polyunsaturated.omega_6();
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutrition {
    energy: Energy,
    carbohydrates: Carbohydrates,
    fat: FatV2,
    protein: Protein,
    salt: Salt,
    #[serde(skip_serializing_if = "Option::is_none")]
    vitamins: Option<Vitamins>,
}

impl Nutrition {
    pub fn new(
        energy: Energy,
        carbohydrates: Carbohydrates,
        fat: FatV2,
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
        return &self.energy;
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        return &self.carbohydrates;
    }

    pub fn fat(&self) -> &FatV2 {
        return &self.fat;
    }

    pub fn protein(&self) -> &Protein {
        return &self.protein;
    }

    pub fn salt(&self) -> &Salt {
        return &self.salt;
    }

    pub fn vitamins(&self) -> Option<Vitamins> {
        self.vitamins.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FatV2 {
    total: f64,
    saturated: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    unsaturated: Option<UnsaturatedFat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trans: Option<f64>,
}

impl FatV2 {
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
        self.unsaturated.clone()
    }

    pub fn trans(&self) -> Option<f64> {
        self.trans.clone()
    }
}

impl TotalAble for FatV2 {
    fn total(&self) -> f64 {
        return self.total;
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
        self.mono.clone()
    }
    pub fn poly(&self) -> Option<PolyUnsaturatedFat> {
        self.poly.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
        self.omega_7.clone()
    }
    pub fn omega_9(&self) -> Option<f64> {
        self.omega_9.clone()
    }
}

impl TotalAble for MonoUnsaturatedFat {
    fn total(&self) -> f64 {
        self.total
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
        self.omega_3.clone()
    }

    pub fn omega_6(&self) -> Option<f64> {
        self.omega_6.clone()
    }
}

impl TotalAble for PolyUnsaturatedFat {
    fn total(&self) -> f64 {
        self.total
    }
}
