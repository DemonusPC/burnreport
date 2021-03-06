use serde_derive::{Deserialize, Serialize};
use std::ops::Add;

pub trait FatSolubleApi {
    fn a(&self) -> f64;
    fn d(&self) -> f64;
    fn e(&self) -> f64;
    fn k(&self) -> f64;
}

pub trait WaterSolubleApi {
    fn b1(&self) -> f64;
    fn b2(&self) -> f64;
    fn b3(&self) -> f64;
    fn b5(&self) -> f64;
    fn b6(&self) -> f64;
    fn b7(&self) -> f64;
    fn b9(&self) -> f64;
    fn b12(&self) -> f64;
    fn c(&self) -> f64;
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FatSoluble {
    a: f64,
    d: f64,
    e: f64,
    k: f64,
}

impl FatSoluble {
    pub fn new(a: f64, d: f64, e: f64, k: f64) -> Self {
        Self { a, d, e, k }
    }

    pub fn is_zero(&self) -> bool {
        if self.a == 0.0 && self.d == 0.0 && self.e == 0.0 && self.k == 0.0 {
            return true;
        }

        false
    }
}

impl FatSolubleApi for FatSoluble {
    fn a(&self) -> f64 {
        self.a
    }
    fn d(&self) -> f64 {
        self.d
    }
    fn e(&self) -> f64 {
        self.e
    }
    fn k(&self) -> f64 {
        self.k
    }
}

impl Add for FatSoluble {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            d: self.d + other.d,
            e: self.e + other.e,
            k: self.k + other.k,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct WaterSoluble {
    b1: f64,
    b2: f64,
    b3: f64,
    b5: f64,
    b6: f64,
    b7: f64,
    b9: f64,
    b12: f64,
    c: f64,
}

impl WaterSoluble {
    pub fn new(
        b1: f64,
        b2: f64,
        b3: f64,
        b5: f64,
        b6: f64,
        b7: f64,
        b9: f64,
        b12: f64,
        c: f64,
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
        if self.b1 == 0.0
            && self.b2 == 0.0
            && self.b3 == 0.0
            && self.b5 == 0.0
            && self.b5 == 0.0
            && self.b6 == 0.0
            && self.b7 == 0.0
            && self.b9 == 0.0
            && self.b12 == 0.0
            && self.c == 0.0
        {
            return true;
        }

        false
    }
}

impl WaterSolubleApi for WaterSoluble {
    fn b1(&self) -> f64 {
        self.b1
    }
    fn b2(&self) -> f64 {
        self.b2
    }
    fn b3(&self) -> f64 {
        self.b3
    }
    fn b5(&self) -> f64 {
        self.b5
    }
    fn b6(&self) -> f64 {
        self.b6
    }
    fn b7(&self) -> f64 {
        self.b7
    }
    fn b9(&self) -> f64 {
        self.b9
    }
    fn b12(&self) -> f64 {
        self.b12
    }
    fn c(&self) -> f64 {
        self.c
    }
}

impl Add for WaterSoluble {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            b1: self.b1 + other.b1,
            b2: self.b2 + other.b2,
            b3: self.b3 + other.b3,
            b5: self.b5 + other.b5,
            b6: self.b6 + other.b6,
            b7: self.b7 + other.b7,
            b9: self.b9 + other.b9,
            b12: self.b12 + other.b12,
            c: self.c + other.c,
        }
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
    fn a(&self) -> f64 {
        self.fat.a()
    }
    fn d(&self) -> f64 {
        self.fat.d()
    }
    fn e(&self) -> f64 {
        self.fat.e()
    }
    fn k(&self) -> f64 {
        self.fat.k()
    }
}

impl WaterSolubleApi for Vitamins {
    fn b1(&self) -> f64 {
        self.water.b1()
    }
    fn b2(&self) -> f64 {
        self.water.b2()
    }
    fn b3(&self) -> f64 {
        self.water.b3()
    }
    fn b5(&self) -> f64 {
        self.water.b5()
    }
    fn b6(&self) -> f64 {
        self.water.b6()
    }
    fn b7(&self) -> f64 {
        self.water.b7()
    }
    fn b9(&self) -> f64 {
        self.water.b9()
    }
    fn b12(&self) -> f64 {
        self.water.b12()
    }
    fn c(&self) -> f64 {
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

pub struct Minerals {
    beta_carotene: f64,
    calcium: f64,
    chromium: f64,
    cobalt: f64,
    ionide: f64,
    magnesium: f64,
    manganese: f64,
    molybdenum: f64,
    phosphorus: f64,
    potassium: f64,
    selenium: f64,
    zinc: f64,
}
