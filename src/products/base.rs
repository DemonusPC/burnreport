use crate::nutrients::{Carbohydrates, Energy, Fat, Nutrients, Protein, Salt, Vitamins};
use actix_web::{HttpRequest, HttpResponse, Responder};
use log::error;
use serde_derive::{Deserialize, Serialize};

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

impl Responder for Portion {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Unit {
    Grams,
    Mililiters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: i32,
    name: String,
    nutrients: Nutrients,
    unit: Unit,
}

impl Product {
    pub fn new(id: i32, name: String, nutrition: Nutrients, unit: Unit) -> Self {
        Self {
            id,
            name,
            nutrients: nutrition,
            unit,
        }
    }
    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients.clone()
    }

    pub fn energy(&self) -> &Energy {
        return &self.nutrients.energy();
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        return &self.nutrients.carbohydrates();
    }

    pub fn fat(&self) -> &Fat {
        return &self.nutrients.fat();
    }

    pub fn protein(&self) -> &Protein {
        return &self.nutrients.protein();
    }

    pub fn salt(&self) -> &Salt {
        return &self.nutrients.salt();
    }

    pub fn vitamins(&self) -> Option<Vitamins> {
        self.nutrients.vitamins()
    }
}

impl Responder for Product {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = match serde_json::to_string(&self) {
            Ok(v) => v,
            Err(error) => {
                error!("Failed to serialize the Product with error: {}", error);
                return HttpResponse::InternalServerError().finish();
            }
        };
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlatProduct {
    // pub id: i32,
    pub name: String,
    pub unit: String,
    pub kj: f64,
    pub kcal: f64,
    pub carbohydrates: f64,
    pub sugar: f64,
    pub fiber: Option<f64>,
    pub added_sugar: Option<f64>,
    pub starch: Option<f64>,
    pub fat: f64,
    pub saturated: f64,
    pub monounsaturated: Option<f64>,
    pub omega_7: Option<f64>,
    pub omega_9: Option<f64>,
    pub polyunsaturated: Option<f64>,
    pub omega_3: Option<f64>,
    pub omega_6: Option<f64>,
    pub trans: f64,
    pub protein: f64,
    pub salt: f64,
    // vitamins
    pub a: Option<f64>,
    pub d: Option<f64>,
    pub e: Option<f64>,
    pub k: Option<f64>,
    pub b1: Option<f64>,
    pub b2: Option<f64>,
    pub b3: Option<f64>,
    pub b5: Option<f64>,
    pub b6: Option<f64>,
    pub b7: Option<f64>,
    pub b9: Option<f64>,
    pub b12: Option<f64>,
    pub c: Option<f64>,
}
