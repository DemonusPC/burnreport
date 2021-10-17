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

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Carbohydrates {
    total: f64,
    sugar: f64,
    fiber: Option<f64>,
    #[serde(rename = "addedSugar")]
    added_sugar: Option<f64>,
    starch: Option<f64>,
}

impl Carbohydrates {
    pub fn new(total: f64, sugar: f64) -> Carbohydrates {
        Carbohydrates {
            total,
            sugar,
            ..Default::default()
        }
    }

    pub fn with_fiber<'a>(&'a mut self, fiber: Option<f64>) -> &'a mut Carbohydrates {
        self.fiber = fiber;
        self
    }
    pub fn with_added_sugar<'a>(&'a mut self, added_sugar: Option<f64>) -> &'a mut Carbohydrates {
        self.added_sugar = added_sugar;
        self
    }
    pub fn with_starch<'a>(&'a mut self, starch: Option<f64>) -> &'a mut Carbohydrates {
        self.starch = starch;
        self
    }

    pub fn build(self) -> Carbohydrates {
        self
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
            sugar: self.sugar + other.sugar,
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

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Fat {
    total: f64,
    saturated: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    unsaturated: Option<UnsaturatedFat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trans: Option<f64>,
}

impl Fat {
    pub fn new(total: f64, saturated: f64) -> Self {
        Self {
            total,
            saturated,
            ..Default::default()
        }
    }

    pub fn with_unsaturated<'a>(&'a mut self, unsaturated: Option<UnsaturatedFat>) -> &'a mut Fat {
        self.unsaturated = unsaturated;
        self
    }

    pub fn with_trans<'a>(&'a mut self, trans: Option<f64>) -> &'a mut Fat {
        self.trans = trans;
        self
    }

    pub fn build(self) -> Fat {
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_basic_nutrients() {
        let one = Nutrients::new(
            Energy::new(55.2, 19.0),
            Carbohydrates::new(20.2, 19.2),
            Fat::new(5.0, 0.0),
            Protein::new(44.9),
            Salt::new(0.02),
            Option::None,
        );
        let two = Nutrients::new(
            Energy::new(125.0, 20.0),
            Carbohydrates::new(5.1, 0.2),
            Fat::new(33.0, 10.0),
            Protein::new(0.0),
            Salt::new(0.003),
            Option::None,
        );

        let expected = Nutrients::new(
            Energy::new(180.2, 39.0),
            Carbohydrates::new(25.2, 19.4),
            Fat::new(38.0, 10.0),
            Protein::new(44.9),
            Salt::new(0.023),
            Option::None,
        );

        let result = one + two;

        assert_eq!(result.energy().kcal(), expected.energy().kcal());
        assert_eq!(result.energy().k_j(), expected.energy().k_j());
        assert_eq!(
            result.carbohydrates().total().round(),
            expected.carbohydrates().total().round()
        );
        assert_eq!(
            result.carbohydrates().sugar(),
            expected.carbohydrates().sugar()
        );
        assert_eq!(result.fat().total(), expected.fat().total());
        assert_eq!(result.fat().saturated(), expected.fat().saturated());
        assert_eq!(
            result.protein().total().round(),
            expected.protein().total().round()
        );
        assert_eq!(result.salt().total(), expected.salt().total());
    }

    #[test]
    fn can_add_complex_carbohydrates() {
        let one = Carbohydrates::new(20.0, 2.2)
            .with_fiber(Some(1.5))
            .with_starch(Some(12.5))
            .build();
        let two = Carbohydrates::new(53.8, 0.0)
            .with_added_sugar(Some(100.0))
            .with_starch(Some(12.5))
            .build();

        let result = one + two;

        assert_eq!(result.total(), 73.8);
        assert_eq!(result.sugar(), 2.2);
        assert_eq!(result.fiber().unwrap(), 1.5);
        assert_eq!(result.added_sugar().unwrap(), 100.0);
        assert_eq!(result.starch().unwrap(), 25.0);
    }

    #[test]
    fn can_add_complex_fats() {
        let mono = MonoUnsaturatedFat::new(12.5, Some(0.5), Some(1.5));
        let poly = PolyUnsaturatedFat::new(15.0, Some(10.0), Some(5.0));
        let unsaturated = UnsaturatedFat::new(Some(mono), Some(poly));

        let one = Fat::new(50.0, 10.5)
            .with_unsaturated(Some(unsaturated))
            .with_trans(Some(5.0))
            .build();
        let two = one.clone();

        let result = one + two;

        assert_eq!(result.total(), 100.0);
        assert_eq!(result.saturated(), 21.0);
        let res_mono = result.unsaturated().unwrap().mono().unwrap();
        let res_poly = result.unsaturated().unwrap().poly().unwrap();
        assert_eq!(res_mono.total(), 25.0);
        assert_eq!(res_mono.omega_7().unwrap(), 1.0);
        assert_eq!(res_mono.omega_9().unwrap(), 3.0);
        assert_eq!(res_poly.total(), 30.0);
        assert_eq!(res_poly.omega_3().unwrap(), 20.0);
        assert_eq!(res_poly.omega_6().unwrap(), 10.0);
    }
}
