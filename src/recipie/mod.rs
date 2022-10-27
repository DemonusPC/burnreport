use serde_derive::{Deserialize, Serialize};

use crate::{product::{Product, Unit}, nutrients::Nutrients};
#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    product: Product,
    amount: f64,
}

impl Ingredient {
    pub fn product(&self) -> &Product {
        &self.product
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipie {
    id: i32,
    name: String,
    ingredients: Vec<Ingredient>,
    total: Nutrients
}

impl Recipie {
    pub fn new(id: i32, name: String, ingredients: Vec<Ingredient>) -> Self {

        let total = ingredients.as_slice().into_iter().fold(Nutrients::empty(), |acc,x|{
            let add = acc + x.product().nutrients();
            add
        });


        Self {
            id,
            name,
            ingredients,
            total 
        }
    }
}