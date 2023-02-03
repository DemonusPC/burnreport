use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

use crate::nutrients::Nutrients;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct StandardProductIdentifier {
    numeric_code: i64,
    alphabetic_code: String,
    name: String,
}

impl StandardProductIdentifier {
    pub fn new(numeric_code: i64, alphabetic_code: &str, name: &str) -> Self {
        Self {
            numeric_code,
            alphabetic_code: alphabetic_code.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn numeric_code(&self) -> i64 {
        self.numeric_code
    }
    pub fn alphabetic_code(&self) -> &str {
        &self.alphabetic_code
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct StandardProductIdentifierRepo {}

impl StandardProductIdentifierRepo {
    pub async fn get_by_numeric_code(
        pool: &SqlitePool,
        code: i64,
    ) -> Result<StandardProductIdentifier, sqlx::Error> {
        let result = sqlx::query(
            "SELECT numeric_code, alphabetic_code, name FROM SPI WHERE numeric_code = ?",
        )
        .bind(code)
        .map(|row: SqliteRow| {
            StandardProductIdentifier::new(
                row.get("numeric_code"),
                row.get("alphabetic_code"),
                row.get("name"),
            )
        })
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_alphabetic_code(
        pool: &SqlitePool,
        alpha_code: &str,
    ) -> Result<StandardProductIdentifier, sqlx::Error> {
        let result = sqlx::query(
            "SELECT numeric_code, alphabetic_code, name FROM SPI WHERE alphabetic_code = ?",
        )
        .bind(alpha_code)
        .map(|row: SqliteRow| {
            StandardProductIdentifier::new(
                row.get("numeric_code"),
                row.get("alphabetic_code"),
                row.get("name"),
            )
        })
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
    pub async fn save(
        pool: &SqlitePool,
        spi: &StandardProductIdentifier,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            r#"INSERT INTO SPI ("numeric_code", "alphabetic_code", "name") VALUES (?1, ?2, ?3);"#,
        )
        .bind(spi.numeric_code())
        .bind(spi.alphabetic_code())
        .bind(spi.name())
        .execute(&mut tx)
        .await?;

        tx.commit().await
    }

    pub async fn average_nutrition_by_numeric_code(code: i64) -> Nutrients {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::config::setup;
    use crate::nutrients::{Carbohydrates, Energy, Fat, Nutrients, Protein, Salt};
    use crate::product::{Product, ProductStore, Unit};

    use super::{StandardProductIdentifier, StandardProductIdentifierRepo};

    #[actix_web::test]
    async fn can_save_and_get_spi() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let spi = StandardProductIdentifier::new(562, "UNBTTR", "Unsalted Butter");

        StandardProductIdentifierRepo::save(&pool, &spi)
            .await
            .unwrap();

        let returned_spi_by_code = StandardProductIdentifierRepo::get_by_numeric_code(&pool, 562)
            .await
            .unwrap();
        let returned_spi_by_alpha_code =
            StandardProductIdentifierRepo::get_by_alphabetic_code(&pool, "UNBTTR")
                .await
                .unwrap();

        assert_eq!(spi, returned_spi_by_code);
        assert_eq!(spi, returned_spi_by_alpha_code);
    }

    // Create a test for generating average / median nutritional values

    #[actix_web::test]
    async fn can_calculate_nutritional_info_for_spi() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let spi = StandardProductIdentifier::new(562, "UNBTTR", "Unsalted Butter");

        let one = Nutrients::new(
            Energy::new(10.0, 10.0),
            Carbohydrates::new(10.0, 10.0),
            Fat::new(5.0, 0.0),
            Protein::new(100.0),
            Salt::new(0.05),
            Option::None,
        );
        let two = Nutrients::new(
            Energy::new(100.0, 100.0),
            Carbohydrates::new(10.0, 10.0),
            Fat::new(100.0, 10.0),
            Protein::new(0.0),
            Salt::new(0.1),
            Option::None,
        );
        let product_one = Product::new_with_spi(
            32,
            "Product One".to_owned(),
            one,
            Unit::Grams,
            Some(spi.clone()),
        );

        let product_two = Product::new_with_spi(
            33,
            "Product Two".to_owned(),
            two,
            Unit::Grams,
            Some(spi.clone()),
        );

        ProductStore::insert_product(&pool, product_one)
            .await
            .unwrap();
        ProductStore::insert_product(&pool, product_two)
            .await
            .unwrap();

        let average = Nutrients::new(
            Energy::new(55.0, 55.0),
            Carbohydrates::new(10.0, 10.0),
            Fat::new(52.5, 5.0),
            Protein::new(50.0),
            Salt::new(0.075),
            Option::None,
        );

        StandardProductIdentifierRepo::save(&pool, &spi)
            .await
            .unwrap();

        let result = StandardProductIdentifierRepo::average_nutrition_by_numeric_code(562).await;

        assert_eq!(result, average);
    }
}
