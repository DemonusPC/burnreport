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
#[serde(rename_all = "camelCase")]
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
            match ProductStore::amount_adjusted_product(&pool, v.numeric_identifier, v.amount).await
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
        if v.entity == ProductEntity::Spi {
            println!("Entity");
            match ProductStore::amount_adjusted_spi_products(&pool, v.numeric_identifier, v.amount)
                .await
            {
                Ok(products) => {
                    if products.len() == 0 {
                        continue;
                    }
                    let mut total_for_spi = Nutrients::default();
                    let mut products_for_spi = 0.0;
                    for p in products {
                        total_for_spi = total_for_spi + p.nutrients();
                        // TODO: Add a product representing an SPI
                        products_for_spi = products_for_spi + 1.0;
                    }
                    total = total + (total_for_spi / products_for_spi);
                }
                Err(err) => {
                    error!(
                        "Failed to return amount adjusted product for spi due to error: {}",
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
        nutrients::{
            Carbohydrates, Energy, Fat, FatSoluble, Nutrients, Protein, Salt, Vitamins,
            WaterSoluble,
        },
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

        let vitamins_example = Vitamins::new(
            FatSoluble::new(Some(100.00), None, None, None),
            WaterSoluble::default(),
        );
        let nutrition_example = Nutrients::new(
            Energy::new(100.0, 100.0),
            Carbohydrates::new(100.0, 100.0),
            Fat::new(100.0, 100.0),
            Protein::new(100.0),
            Salt::new(100.0),
            Some(vitamins_example.clone()),
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
            spi: Some(spi.numeric_code()),
        };

        let ingredient_id_one = ProductStore::insert_product(&pool, ingredient_one)
            .await
            .unwrap();
        let ingredient_id_two = ProductStore::insert_product(&pool, ingredient_two)
            .await
            .unwrap();

        let report = ReportRequest {
            consumed: vec![
                ReportItem {
                    entity: ProductEntity::Product,
                    numeric_identifier: ingredient_id_one,
                    amount: 100.0,
                },
                ReportItem {
                    entity: ProductEntity::Spi,
                    numeric_identifier: 866,
                    amount: 25.0,
                },
            ],
        };

        let expected_total = Nutrients::new(
            Energy::new(125.0, 125.0),
            Carbohydrates::new(125.0, 125.0),
            Fat::new(125.0, 125.0),
            Protein::new(125.0),
            Salt::new(125.0),
            Some(Vitamins::new(
                FatSoluble::new(Some(125.00), None, None, None),
                WaterSoluble::default(),
            )),
        );

        let result = run_report(&pool, report).await.unwrap();

        let expected_report = ReportResult {
            total: expected_total,
            consumed: vec![],
        };

        assert_eq!(result.total, expected_report.total);
    }
}
