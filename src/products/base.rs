use crate::nutrients::{Carbohydrates, Energy, Fat, FatV2, Protein, Salt};
use crate::nutrients::{Nutrition, Vitamins};
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
    nutrition: Nutrition,
    unit: Unit,
}

impl Product {
    pub fn new(id: i32, name: String, nutrition: Nutrition, unit: Unit) -> Self {
        Self {
            id,
            name,
            nutrition,
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

    pub fn energy(&self) -> &Energy {
        return &self.nutrition.energy();
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        return &self.nutrition.carbohydrates();
    }

    pub fn fat(&self) -> &FatV2 {
        return &self.nutrition.fat();
    }

    pub fn protein(&self) -> &Protein {
        return &self.nutrition.protein();
    }

    pub fn salt(&self) -> &Salt {
        return &self.nutrition.salt();
    }

    pub fn vitamins(&self) -> Option<Vitamins> {
        self.nutrition.vitamins()
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
