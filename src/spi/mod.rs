use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

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

pub struct StandardProductIdentifierStore {}

impl StandardProductIdentifierStore {
    pub async fn get_by_numeric_code(
        pool: &SqlitePool,
        code: i64,
    ) -> Result<StandardProductIdentifier, sqlx::Error> {
        let result = sqlx::query(
            "SELECT numeric_code, alphabetic_code, full_name FROM SPI WHERE numeric_code = ?",
        )
        .bind(code)
        .map(|row: SqliteRow| {
            StandardProductIdentifier::new(
                row.get("numeric_code"),
                row.get("alphabetic_code"),
                row.get("full_name"),
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
            "SELECT numeric_code, alphabetic_code, full_name FROM SPI WHERE alphabetic_code = ?",
        )
        .bind(alpha_code)
        .map(|row: SqliteRow| {
            StandardProductIdentifier::new(
                row.get("numeric_code"),
                row.get("alphabetic_code"),
                row.get("full_name"),
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
            r#"INSERT INTO SPI ("numeric_code", "alphabetic_code", "full_name") VALUES (?1, ?2, ?3);"#,
        )
        .bind(spi.numeric_code())
        .bind(spi.alphabetic_code())
        .bind(spi.name())
        .execute(&mut tx)
        .await?;

        tx.commit().await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<StandardProductIdentifier>, sqlx::Error> {
        let result = sqlx::query("SELECT numeric_code, alphabetic_code, full_name FROM SPI;")
            .map(|row: SqliteRow| {
                return StandardProductIdentifier {
                    numeric_code: row.get("numeric_code"),
                    alphabetic_code: row.get("alphabetic_code"),
                    name: row.get("full_name"),
                };
            })
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
    pub async fn delete_by_numeric_code(pool: &SqlitePool, code: i64) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("DELETE FROM SPI WHERE numeric_code = ?1")
            .bind(code)
            .execute(&mut tx)
            .await?;

        tx.commit().await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::config::setup;

    use super::{StandardProductIdentifier, StandardProductIdentifierStore};

    #[actix_web::test]
    async fn can_perform_all_operations_for_spi() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let spi = StandardProductIdentifier::new(562, "UNBTTR", "Unsalted Butter");

        StandardProductIdentifierStore::save(&pool, &spi)
            .await
            .unwrap();

        let returned_spi_by_code = StandardProductIdentifierStore::get_by_numeric_code(&pool, 562)
            .await
            .unwrap();
        let returned_spi_by_alpha_code =
            StandardProductIdentifierStore::get_by_alphabetic_code(&pool, "UNBTTR")
                .await
                .unwrap();

        assert_eq!(spi, returned_spi_by_code);
        assert_eq!(spi, returned_spi_by_alpha_code);

        let list = StandardProductIdentifierStore::list(&pool).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(&spi, &list[0]);

        StandardProductIdentifierStore::delete_by_numeric_code(&pool, 562)
            .await
            .unwrap();

        assert_eq!(
            StandardProductIdentifierStore::list(&pool)
                .await
                .unwrap()
                .len(),
            0
        );
    }
}
