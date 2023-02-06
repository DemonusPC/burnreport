use std::fmt;

use log::error;
use serde_derive::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    nutrients::Nutrients,
    product::{Product, ProductStore},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSubmission {
    id: i32,
    name: String,
    amount: f64,
}

impl ProductSubmission {
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

#[derive(Debug, Deserialize, PartialEq)]
pub enum ProductEntity {
    Product,
    Spi,
}

#[derive(Debug, Deserialize)]
pub struct ReportItem {
    pub entity: ProductEntity,
    pub numeric_identifier: i64,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    pub consumed: Vec<ReportItem>,
}

#[derive(Debug, Clone)]
pub struct ReportError;

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Report processing failed")
    }
}

#[derive(Debug, Serialize)]
pub struct ReportResult {
    pub total: Nutrients,
    pub consumed: Vec<Product>,
}

pub async fn run_report(
    pool: &SqlitePool,
    report: ReportRequest,
) -> Result<ReportResult, ReportError> {
    let mut result: Vec<Product> = vec![];

    let mut total = Nutrients::default();

    for v in &report.consumed {
        if v.entity == ProductEntity::Product {
            match ProductStore::amount_adjusted_product_v2(&pool, v.numeric_identifier, v.amount)
                .await
            {
                Ok(product) => {
                    total = total + product.nutrients();
                    result.push(product);
                }
                Err(err) => {
                    error!(
                        "Failed to return amount adjusted product due to error: {}",
                        err
                    );
                    return Err(ReportError);
                }
            }
        }
    }

    Ok(ReportResult {
        total,
        consumed: result,
    })
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::{
        config::setup,
        nutrients::{Carbohydrates, Energy, Fat, Nutrients, Protein, Salt},
        product::{CreateProductRequest, ProductStore, Unit},
        spi::{StandardProductIdentifier, StandardProductIdentifierStore},
    };

    use super::{run_report, ProductEntity, ReportItem, ReportRequest, ReportResult};

    #[actix_web::test]
    async fn can_run_a_report() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let spi = StandardProductIdentifier::new(866, "IG2", "Ingredient 2");

        StandardProductIdentifierStore::save(&pool, &spi)
            .await
            .unwrap();

        let nutrition_example = Nutrients::new(
            Energy::new(100.0, 100.0),
            Carbohydrates::new(100.0, 100.0),
            Fat::new(100.0, 100.0),
            Protein::new(100.0),
            Salt::new(100.0),
            None,
        );

        let ingredient_one = CreateProductRequest {
            name: "Ingredient One".to_owned(),
            nutrients: nutrition_example.clone(),
            unit: Unit::Grams,
            spi: None,
        };
        let ingredient_two = CreateProductRequest {
            name: "Ingredient Two".to_owned(),
            nutrients: nutrition_example.clone(),
            unit: Unit::Grams,
            spi: Some(spi),
        };

        let ingredient_id_one = ProductStore::insert_product(&pool, ingredient_one)
            .await
            .unwrap();
        let ingredient_id_two = ProductStore::insert_product(&pool, ingredient_two)
            .await
            .unwrap();

        let report = ReportRequest {
            consumed: vec![ReportItem {
                entity: ProductEntity::Product,
                numeric_identifier: ingredient_id_one,
                amount: 100.0,
            }],
        };

        let expected_total = Nutrients::new(
            Energy::new(100.0, 100.0),
            Carbohydrates::new(100.0, 100.0),
            Fat::new(100.0, 100.0),
            Protein::new(100.0),
            Salt::new(100.0),
            None,
        );

        let result = run_report(&pool, report).await.unwrap();

        let expected_report = ReportResult {
            total: expected_total,
            consumed: vec![],
        };

        assert_eq!(result.total, expected_report.total);
    }
}
