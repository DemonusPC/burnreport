use serde_derive::{Deserialize, Serialize};

pub trait FatSolubleApi {
    fn a(&self) -> i32;
    fn d(&self) -> i32;
    fn e(&self) -> i32;
    fn k(&self) -> i32;
}

pub trait WaterSolubleApi {
    fn b1(&self) -> i32;
    fn b2(&self) -> i32;
    fn b3(&self) -> i32;
    fn b5(&self) -> i32;
    fn b6(&self) -> i32;
    fn b7(&self) -> i32;
    fn b9(&self) -> i32;
    fn b12(&self) -> i32;
    fn c(&self) -> i32;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FatSoluble {
    a: i32,
    d: i32,
    e: i32,
    k: i32,
}

impl FatSolubleApi for FatSoluble {
    fn a(&self) -> i32 {
        self.a
    }
    fn d(&self) -> i32 {
        self.d
    }
    fn e(&self) -> i32 {
        self.e
    }
    fn k(&self) -> i32 {
        self.k
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WaterSoluble {
    b1: i32,
    b2: i32,
    b3: i32,
    b5: i32,
    b6: i32,
    b7: i32,
    b9: i32,
    b12: i32,
    c: i32,
}

impl WaterSolubleApi for WaterSoluble {
    fn b1(&self) -> i32 {
        self.b1
    }
    fn b2(&self) -> i32 {
        self.b2
    }
    fn b3(&self) -> i32 {
        self.b3
    }
    fn b5(&self) -> i32 {
        self.b5
    }
    fn b6(&self) -> i32 {
        self.b6
    }
    fn b7(&self) -> i32 {
        self.b7
    }
    fn b9(&self) -> i32 {
        self.b9
    }
    fn b12(&self) -> i32 {
        self.b12
    }
    fn c(&self) -> i32 {
        self.c
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Vitamins {
    fat: FatSoluble,
    water: WaterSoluble,
}

impl FatSolubleApi for Vitamins {
    fn a(&self) -> i32 {
        self.fat.a()
    }
    fn d(&self) -> i32 {
        self.fat.d()
    }
    fn e(&self) -> i32 {
        self.fat.e()
    }
    fn k(&self) -> i32 {
        self.fat.k()
    }
}

impl WaterSolubleApi for Vitamins {
    fn b1(&self) -> i32 {
        self.water.b1()
    }
    fn b2(&self) -> i32 {
        self.water.b2()
    }
    fn b3(&self) -> i32 {
        self.water.b3()
    }
    fn b5(&self) -> i32 {
        self.water.b5()
    }
    fn b6(&self) -> i32 {
        self.water.b6()
    }
    fn b7(&self) -> i32 {
        self.water.b7()
    }
    fn b9(&self) -> i32 {
        self.water.b9()
    }
    fn b12(&self) -> i32 {
        self.water.b12()
    }
    fn c(&self) -> i32 {
        self.water.c()
    }
}

pub struct Minerals {
    beta_carotene: i32,
    calcium: i32,
    chromium: i32,
    cobalt: i32,
    ionide: i32,
    magnesium: i32,
    manganese: i32,
    molybdenum: i32,
    phosphorus: i32,
    potassium: i32,
    selenium: i32,
    zinc: i32,
}
