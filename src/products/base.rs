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
