use serde_derive::{Deserialize, Serialize};

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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FatSoluble {
    a: f64,
    d: f64,
    e: f64,
    k: f64,
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

#[derive(Default, Debug, Serialize, Deserialize)]
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

// Its important to remember that unlike food the values in the Vitamins table is in mg
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Vitamins {
    fat: FatSoluble,
    water: WaterSoluble,
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
