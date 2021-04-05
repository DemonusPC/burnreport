use crate::nutrients::TotalAble;
use crate::products::{Portion, Product};
use crate::{
    body::{BodyLog, BodyOverview},
    nutrients::{Carbohydrates, Energy, Fat, Protein, Salt},
};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::SqlitePool;

pub async fn import_file(pool: &SqlitePool, products: &[Product]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    for product in products {
        let result = sqlx::query(r#"INSERT INTO Food ( name, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt)
        VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"#)
        .bind(product.name())
        .bind(product.manufacturer())
        .bind(product.energy().kcal())
        .bind(product.energy().k_j())
        .bind(product.carbohydrates().total())
        .bind(product.carbohydrates().fiber())
        .bind(product.carbohydrates().sugar())
        .bind(product.carbohydrates().added_sugar())
        .bind(product.carbohydrates().starch())
        .bind(product.fat().total())
        .bind(product.fat().saturated())
        .bind(product.fat().monounsaturated())
        .bind(product.fat().trans())
        .bind(product.protein().total())
        .bind(product.salt().total())
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

pub async fn search_products(pool: &SqlitePool, term: &str) -> Result<Vec<Product>, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT * FROM Food WHERE name LIKE $1 LIMIT 15")
        .bind(format!("%{}%", term))
        .map(|row: SqliteRow| {
            // let name : String = row.get(0);
            let energy: Energy = Energy::new(row.get(3), row.get(4));
            let carbs: Carbohydrates =
                Carbohydrates::new(row.get(5), row.get(6), row.get(7), row.get(8), row.get(9));
            let fat: Fat = Fat::new(row.get(10), row.get(11), row.get(12), row.get(13));
            let protein: Protein = Protein::new(row.get(14));
            let salt: Salt = Salt::new(row.get(15));
            Product::new(
                row.get(0),
                row.get(1),
                row.get(2),
                energy,
                carbs,
                fat,
                protein,
                salt,
            )
        })
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn single_product(pool: &SqlitePool, id: i32) -> Result<Product, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT * FROM Food WHERE id = ?")
        .bind(id)
        .map(|row: SqliteRow| {
            // let name : String = row.get(0);
            let energy: Energy = Energy::new(row.get(3), row.get(4));
            let carbs: Carbohydrates =
                Carbohydrates::new(row.get(5), row.get(6), row.get(7), row.get(8), row.get(9));
            let fat: Fat = Fat::new(row.get(10), row.get(11), row.get(12), row.get(13));
            let protein: Protein = Protein::new(row.get(14));
            let salt: Salt = Salt::new(row.get(15));
            Product::new(
                row.get(0),
                row.get(1),
                row.get(2),
                energy,
                carbs,
                fat,
                protein,
                salt,
            )
        })
        .fetch_one(pool)
        .await?;
    Ok(result)
}

// SELECT id, manufacturer, (kcal/100) * 20 as kcal, (kj/100) * 20 as kj,  (carbohydrates/100) * 20 as carbohydrates,  (fiber/100) * 20 as fiber, (sugar/100) * 20 as sugar, (added_sugar/100) * 20 as added_sugar,  (starch/100) * 20 as starch, (fat/100) * 20 as fat, (saturated/100) * 20 as saturated, (monounsaturated/100) * 20 as monounsaturated, (trans/100) * 20 as trans, (protein/100) * 20 as protein, (salt/100) * 20 as salt FROM Food WHERE id = 1
pub async fn one_single_product(
    pool: &SqlitePool,
    id: i32,
    amount: f64,
) -> Result<Product, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT id, name, manufacturer, (kcal/100) * $1 as kcal, (kj/100) * $1 as kj,  (carbohydrates/100) * $1 as carbohydrates,  (fiber/100) * $1 as fiber, (sugar/100) * $1 as sugar, (added_sugar/100) * $1 as added_sugar,  (starch/100) * $1 as starch, (fat/100) * $1 as fat, (saturated/100) * $1 as saturated, (monounsaturated/100) * $1 as monounsaturated, (trans/100) * $1 as trans, (protein/100) * $1 as protein, (salt/100) * $1 as salt FROM Food WHERE id = $2")
        .bind(amount)
        .bind(id)
        .map(|row: SqliteRow| {
            // let name : String = row.get(0);
            let energy: Energy = Energy::new(row.get(3), row.get(4));
            let carbs: Carbohydrates =
                Carbohydrates::new(row.get(5), row.get(6), row.get(7), row.get(8), row.get(9));
            let fat: Fat = Fat::new(row.get(10), row.get(11), row.get(12), row.get(13));
            let protein: Protein = Protein::new(row.get(14));
            let salt: Salt = Salt::new(row.get(15));
            Product::new(
                row.get(0),
                row.get(1),
                row.get(2),
                energy,
                carbs,
                fat,
                protein,
                salt,
            )
        })
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn insert_product(pool: &SqlitePool, product: Product) -> Result<i64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(r#"INSERT INTO Food ( name, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt)
    VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"#)
    .bind(product.name())
    .bind(product.manufacturer())
    .bind(product.energy().kcal())
    .bind(product.energy().k_j())
    .bind(product.carbohydrates().total())
    .bind(product.carbohydrates().fiber())
    .bind(product.carbohydrates().sugar())
    .bind(product.carbohydrates().added_sugar())
    .bind(product.carbohydrates().starch())
    .bind(product.fat().total())
    .bind(product.fat().saturated())
    .bind(product.fat().monounsaturated())
    .bind(product.fat().trans())
    .bind(product.protein().total())
    .bind(product.salt().total())
    .execute(&mut tx)
    .await?;

    let rec: (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(result.last_insert_rowid())
}

pub async fn delete_product(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM Food WHERE id = ?")
        .bind(id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

// Portions

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
        sqlx::query!(
            r#"
            INSERT INTO "main"."Portions"
            ("product", "name", "grams")
            VALUES (?1, ?2, ?3);
            "#,
            p,
            name,
            grams
        )
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

// Modify a product Size
// TODO: Add functionality for modifying product sizes
// Lol still didnt do that todo

pub async fn body_overview(pool: &SqlitePool) -> Result<BodyOverview, sqlx::Error> {
    let body_log: Vec<BodyLog> =
        sqlx::query(" SELECT date, mass, fat FROM Body ORDER BY date DESC LIMIT 30;")
            .map(|row: SqliteRow| -> BodyLog {
                let date = row.get(0);
                let mass: f64 = row.get_unchecked(1);
                let fat: f64 = row.get_unchecked(2);
                BodyLog::new(date, mass, fat)
            })
            .fetch_all(pool)
            .await?;

    let result = BodyOverview::new_from_log(body_log);

    Ok(result)
}

pub async fn insert_body_log_db(pool: &SqlitePool, body_log: BodyLog) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let date = body_log.date().date().and_hms_nano(0, 0, 0, 0);
    let mass = body_log.mass();
    let fat = body_log.fat();

    sqlx::query!(
        r#"
        INSERT INTO "main"."Body"
        ("date", "mass", "fat")
        VALUES (?1, ?2, ?3);
        "#,
        date,
        mass,
        fat
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(true)
}

pub async fn update_body_log_db(pool: &SqlitePool, body_log: BodyLog) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let date = body_log.date().date().and_hms_nano(0, 0, 0, 0);
    let mass = body_log.mass();
    let fat = body_log.fat();

    sqlx::query!(
        r#"
        UPDATE "main"."Body" SET mass = ?2, fat = ?3 WHERE date = ?1;
        "#,
        date,
        mass,
        fat
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(true)
}
