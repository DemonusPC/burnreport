use serde_derive::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul};

use super::{add_options, divide_option_by_constant, multiply_option_by_constant};

pub trait FatSolubleApi {
    fn a(&self) -> Option<f64>;
    fn d(&self) -> Option<f64>;
    fn e(&self) -> Option<f64>;
    fn k(&self) -> Option<f64>;
}

pub trait WaterSolubleApi {
    fn b1(&self) -> Option<f64>;
    fn b2(&self) -> Option<f64>;
    fn b3(&self) -> Option<f64>;
    fn b5(&self) -> Option<f64>;
    fn b6(&self) -> Option<f64>;
    fn b7(&self) -> Option<f64>;
    fn b9(&self) -> Option<f64>;
    fn b12(&self) -> Option<f64>;
    fn c(&self) -> Option<f64>;
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FatSoluble {
    a: Option<f64>,
    d: Option<f64>,
    e: Option<f64>,
    k: Option<f64>,
}

impl FatSoluble {
    pub fn new(a: Option<f64>, d: Option<f64>, e: Option<f64>, k: Option<f64>) -> Self {
        Self { a, d, e, k }
    }

    pub fn is_zero(&self) -> bool {
        if self.a.is_none() && self.d.is_none() && self.e.is_none() && self.k.is_none() {
            return true;
        }

        false
    }
}

impl FatSolubleApi for FatSoluble {
    fn a(&self) -> Option<f64> {
        self.a
    }
    fn d(&self) -> Option<f64> {
        self.d
    }
    fn e(&self) -> Option<f64> {
        self.e
    }
    fn k(&self) -> Option<f64> {
        self.k
    }
}

impl Add for FatSoluble {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: add_options(&self.a, &other.a),
            d: add_options(&self.d, &other.d),
            e: add_options(&self.e, &other.e),
            k: add_options(&self.k, &other.k),
        }
    }
}

impl Mul<f64> for FatSoluble {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            a: multiply_option_by_constant(&self.a, rhs),
            d: multiply_option_by_constant(&self.d, rhs),
            e: multiply_option_by_constant(&self.e, rhs),
            k: multiply_option_by_constant(&self.k, rhs),
        }
    }
}

impl Div<f64> for FatSoluble {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            a: divide_option_by_constant(&self.a, rhs),
            d: divide_option_by_constant(&self.d, rhs),
            e: divide_option_by_constant(&self.e, rhs),
            k: divide_option_by_constant(&self.k, rhs),
        }
    }
}

impl PartialEq for FatSoluble {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.d == other.d && self.e == other.e && self.k == other.k
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct WaterSoluble {
    b1: Option<f64>,
    b2: Option<f64>,
    b3: Option<f64>,
    b5: Option<f64>,
    b6: Option<f64>,
    b7: Option<f64>,
    b9: Option<f64>,
    b12: Option<f64>,
    c: Option<f64>,
}

impl WaterSoluble {
    pub fn new(
        b1: Option<f64>,
        b2: Option<f64>,
        b3: Option<f64>,
        b5: Option<f64>,
        b6: Option<f64>,
        b7: Option<f64>,
        b9: Option<f64>,
        b12: Option<f64>,
        c: Option<f64>,
    ) -> Self {
        Self {
            b1,
            b2,
            b3,
            b5,
            b6,
            b7,
            b9,
            b12,
            c,
        }
    }

    // Ugly as fuck and slow but currently no other way.
    // I need to think of a better way of figuring out that
    // vitamins are not present
    pub fn is_zero(&self) -> bool {
        if self.b1.is_none()
            && self.b2.is_none()
            && self.b3.is_none()
            && self.b5.is_none()
            && self.b5.is_none()
            && self.b6.is_none()
            && self.b7.is_none()
            && self.b9.is_none()
            && self.b12.is_none()
            && self.c.is_none()
        {
            return true;
        }

        false
    }
}

impl WaterSolubleApi for WaterSoluble {
    fn b1(&self) -> Option<f64> {
        self.b1
    }
    fn b2(&self) -> Option<f64> {
        self.b2
    }
    fn b3(&self) -> Option<f64> {
        self.b3
    }
    fn b5(&self) -> Option<f64> {
        self.b5
    }
    fn b6(&self) -> Option<f64> {
        self.b6
    }
    fn b7(&self) -> Option<f64> {
        self.b7
    }
    fn b9(&self) -> Option<f64> {
        self.b9
    }
    fn b12(&self) -> Option<f64> {
        self.b12
    }
    fn c(&self) -> Option<f64> {
        self.c
    }
}

impl Add for WaterSoluble {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            b1: add_options(&self.b1, &other.b1),
            b2: add_options(&self.b2, &other.b2),
            b3: add_options(&self.b3, &other.b3),
            b5: add_options(&self.b5, &other.b5),
            b6: add_options(&self.b6, &other.b6),
            b7: add_options(&self.b7, &other.b7),
            b9: add_options(&self.b9, &other.b9),
            b12: add_options(&self.b12, &other.b12),
            c: add_options(&self.c, &other.c),
        }
    }
}

impl Mul<f64> for WaterSoluble {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            b1: multiply_option_by_constant(&self.b1, rhs),
            b2: multiply_option_by_constant(&self.b2, rhs),
            b3: multiply_option_by_constant(&self.b3, rhs),
            b5: multiply_option_by_constant(&self.b5, rhs),
            b6: multiply_option_by_constant(&self.b6, rhs),
            b7: multiply_option_by_constant(&self.b7, rhs),
            b9: multiply_option_by_constant(&self.b9, rhs),
            b12: multiply_option_by_constant(&self.b12, rhs),
            c: multiply_option_by_constant(&self.c, rhs),
        }
    }
}

impl Div<f64> for WaterSoluble {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            b1: divide_option_by_constant(&self.b1, rhs),
            b2: divide_option_by_constant(&self.b2, rhs),
            b3: divide_option_by_constant(&self.b3, rhs),
            b5: divide_option_by_constant(&self.b5, rhs),
            b6: divide_option_by_constant(&self.b6, rhs),
            b7: divide_option_by_constant(&self.b7, rhs),
            b9: divide_option_by_constant(&self.b9, rhs),
            b12: divide_option_by_constant(&self.b12, rhs),
            c: divide_option_by_constant(&self.c, rhs),
        }
    }
}

impl PartialEq for WaterSoluble {
    fn eq(&self, other: &Self) -> bool {
        self.b1 == other.b1
            && self.b2 == other.b2
            && self.b3 == other.b3
            && self.b5 == other.b5
            && self.b6 == other.b6
            && self.b7 == other.b7
            && self.b9 == other.b9
            && self.b12 == other.b12
            && self.c == other.c
    }
}

// Its important to remember that unlike food the values in the Vitamins table is in mg
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Vitamins {
    fat: FatSoluble,
    water: WaterSoluble,
}

impl Vitamins {
    pub fn new(fat: FatSoluble, water: WaterSoluble) -> Self {
        Self { fat, water }
    }

    pub fn is_zero(&self) -> bool {
        self.fat.is_zero() && self.water.is_zero()
    }
}

impl FatSolubleApi for Vitamins {
    fn a(&self) -> Option<f64> {
        self.fat.a()
    }
    fn d(&self) -> Option<f64> {
        self.fat.d()
    }
    fn e(&self) -> Option<f64> {
        self.fat.e()
    }
    fn k(&self) -> Option<f64> {
        self.fat.k()
    }
}

impl WaterSolubleApi for Vitamins {
    fn b1(&self) -> Option<f64> {
        self.water.b1()
    }
    fn b2(&self) -> Option<f64> {
        self.water.b2()
    }
    fn b3(&self) -> Option<f64> {
        self.water.b3()
    }
    fn b5(&self) -> Option<f64> {
        self.water.b5()
    }
    fn b6(&self) -> Option<f64> {
        self.water.b6()
    }
    fn b7(&self) -> Option<f64> {
        self.water.b7()
    }
    fn b9(&self) -> Option<f64> {
        self.water.b9()
    }
    fn b12(&self) -> Option<f64> {
        self.water.b12()
    }
    fn c(&self) -> Option<f64> {
        self.water.c()
    }
}

impl Add for Vitamins {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            fat: self.fat + other.fat,
            water: self.water + other.water,
        }
    }
}

impl Mul<f64> for Vitamins {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            fat: self.fat * rhs,
            water: self.water * rhs,
        }
    }
}

impl Div<f64> for Vitamins {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            fat: self.fat / rhs,
            water: self.water / rhs,
        }
    }
}

impl PartialEq for Vitamins {
    fn eq(&self, other: &Self) -> bool {
        self.fat == other.fat && self.water == other.water
    }
}

pub struct Minerals {
    beta_carotene: Option<f64>,
    calcium: Option<f64>,
    chromium: Option<f64>,
    cobalt: Option<f64>,
    ionide: Option<f64>,
    magnesium: Option<f64>,
    manganese: Option<f64>,
    molybdenum: Option<f64>,
    phosphorus: Option<f64>,
    potassium: Option<f64>,
    selenium: Option<f64>,
    zinc: Option<f64>,
}
