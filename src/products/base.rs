use crate::nutrients::{Carbohydrates, Energy, Fat, Protein, Salt};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: i32,
    name: String,
    manufacturer: String,
    energy: Energy,
    carbohydrates: Carbohydrates,
    fat: Fat,
    protein: Protein,
    salt: Salt,
}

impl Product {
    pub fn new(
        id: i32,
        name: String,
        manufacturer: String,
        energy: Energy,
        carbohydrates: Carbohydrates,
        fat: Fat,
        protein: Protein,
        salt: Salt,
    ) -> Product {
        return Product {
            id: id,
            name: name,
            manufacturer: manufacturer,
            energy: energy,
            carbohydrates: carbohydrates,
            fat: fat,
            protein: protein,
            salt: salt,
        };
    }

    pub fn new_from_raw_values(
        id: i32,
        name: String,
        manufacturer: String,
        kcal: f64,
        k_j: f64,
        total_carbs: f64,
        fiber: f64,
        sugar: f64,
        added_sugar: f64,
        starch: f64,
        total_fat: f64,
        saturated: f64,
        mono: f64,
        trans: f64,
        protein: f64,
        salt: f64,
    ) -> Product {
        return Product {
            id: id,
            name: name,
            manufacturer: manufacturer,
            energy: Energy::new(kcal, k_j),
            carbohydrates: Carbohydrates::new(total_carbs, fiber, sugar, added_sugar, starch),
            fat: Fat::new(total_fat, saturated, mono, trans),
            protein: Protein::new(protein),
            salt: Salt::new(salt),
        };
    }

    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn manufacturer(&self) -> String {
        return self.manufacturer.clone();
    }

    pub fn energy(&self) -> &Energy {
        return &self.energy;
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        return &self.carbohydrates;
    }

    pub fn fat(&self) -> &Fat {
        return &self.fat;
    }

    pub fn protein(&self) -> &Protein {
        return &self.protein;
    }

    pub fn salt(&self) -> &Salt {
        return &self.salt;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSubmission {
    id: i32,
    name: String,
    amount: f64,
}

impl ProductSubmission {
    pub fn new(id: i32, name: String, amount: f64) -> ProductSubmission {
        ProductSubmission { id, name, amount }
    }

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Portion {
    #[serde(skip_deserializing)]
    id: i32,
    product: i32,
    name: String,
    grams: f64,
}

impl Portion {
    pub fn new(id: i32, product: i32, name: String, grams: f64) -> Self {
        Portion {
            id: id,
            product: product,
            name: name,
            grams: grams,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn product(&self) -> i32 {
        self.product
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn grams(&self) -> f64 {
        self.grams
    }
}

impl Default for Portion {
    fn default() -> Self {
        Portion {
            id: 0,
            product: -1,
            name: "".to_owned(),
            grams: 0.0,
        }
    }
}
