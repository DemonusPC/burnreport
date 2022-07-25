use serde_derive::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::SqlitePool;

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
            id,
            product,
            name,
            grams,
        }
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

pub async fn list_portions(
    pool: &SqlitePool,
    product_id: i32,
) -> Result<Vec<Portion>, sqlx::Error> {
    let result = sqlx::query("SELECT id, product, name, grams FROM Portions WHERE product = $1")
        .bind(product_id)
        .map(|row: SqliteRow| Portion::new(row.get(0), row.get(1), row.get(2), row.get(3)))
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn insert_portion(
    pool: &SqlitePool,
    product_sizes: Vec<Portion>,
) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    for size in product_sizes {
        let p = size.product();
        let name = size.name();
        let grams = size.grams();
        sqlx::query(
            r#"
        INSERT INTO "Portions"
        ("product", "name", "grams")
        VALUES (?1, ?2, ?3);
        "#,
        )
        .bind(p)
        .bind(name)
        .bind(grams)
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(true)
}

pub async fn remove_portion(
    pool: &SqlitePool,
    product: i32,
    name: &str,
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(r#"DELETE FROM Portions WHERE product = ?1 AND name = ?2"#)
        .bind(product)
        .bind(name)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;
    Ok(1)
}
