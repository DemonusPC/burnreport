use crate::nutrients::FatSoluble;
use crate::nutrients::TotalAble;
use crate::nutrients::WaterSoluble;
use crate::nutrients::{FatSolubleApi, Vitamins, WaterSolubleApi};
use crate::products::{Portion, Product, SearchSuggestion};
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

pub async fn search_product_suggestions(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT id, name, manufacturer FROM Food WHERE name LIKE $1 LIMIT 15")
        .bind(format!("%{}%", term))
        .map(|row: SqliteRow| {
            let id: i32 = row.get(0);
            let text: String = row.get(1);
            let sub_text: String = row.get(2);
            SearchSuggestion::new(id, text, Some(sub_text), Some("Product".to_owned()))
        })
        .fetch_all(pool)
        .await?;
    Ok(result)
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
                Option::None,
            )
        })
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn single_product(pool: &SqlitePool, id: i32) -> Result<Product, sqlx::Error> {
    // For now we will do a vitamins call before the product call jus to make it cleaner
    // (This isnt the fastest way)
    let vitamins = match sqlx::query("SELECT * FROM Vitamins WHERE product = ?")
        .bind(id)
        .map(|row: SqliteRow| {
            let fat = FatSoluble::new(
                row.try_get(2).unwrap_or(0.0),
                row.try_get(3).unwrap_or(0.0),
                row.try_get(4).unwrap_or(0.0),
                row.try_get(5).unwrap_or(0.0),
            );
            let water = WaterSoluble::new(
                row.try_get(6).unwrap_or(0.0),
                row.try_get(7).unwrap_or(0.0),
                row.try_get(8).unwrap_or(0.0),
                row.try_get(9).unwrap_or(0.0),
                row.try_get(10).unwrap_or(0.0),
                row.try_get(11).unwrap_or(0.0),
                row.try_get(12).unwrap_or(0.0),
                row.try_get(13).unwrap_or(0.0),
                row.try_get(14).unwrap_or(0.0),
            );

            Vitamins::new(fat, water)
        })
        .fetch_one(pool)
        .await
    {
        Ok(v) => Some(v),
        Err(_err) => Option::None,
    };

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
                // We only fetch one so we will only clone once
                vitamins.clone(),
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
    let result = sqlx::query(
        r#"SELECT 
                                id, 
                                name, 
                                manufacturer, 
                                (kcal/100) * $1 as kcal,
                                (kj/100) * $1 as kj,
                                (carbohydrates/100) * $1 as carbohydrates,  
                                (fiber/100) * $1 as fiber, 
                                (sugar/100) * $1 as sugar, 
                                (added_sugar/100) * $1 as added_sugar,  
                                (starch/100) * $1 as starch, 
                                (fat/100) * $1 as fat, 
                                (saturated/100) * $1 as saturated, 
                                (monounsaturated/100) * $1 as monounsaturated, 
                                (trans/100) * $1 as trans, 
                                (protein/100) * $1 as protein, 
                                (salt/100) * $1 as salt, 
                                (a/100) * $1 as a, 
                                (d/100) * $1 as d, 
                                (e/100) * $1 as e, 
                                (k/100) * $1 as k,
                                (b1/100) * $1 as b1,
                                (b2/100) * $1 as b2,
                                (b3/100) * $1 as b3,
                                (b5/100) * $1 as b5,
                                (b6/100) * $1 as b6,
                                (b7/100) * $1 as b7,
                                (b9/100) * $1 as b9,
                                (b12/100) * $1 as b12,
                                (c/100) * $1 as c
                                FROM full_product WHERE id = $2"#,
    )
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

        let fat_sol = FatSoluble::new(
            row.try_get(16).unwrap_or(0.0),
            row.try_get(17).unwrap_or(0.0),
            row.try_get(18).unwrap_or(0.0),
            row.try_get(19).unwrap_or(0.0),
        );
        let water_sol = WaterSoluble::new(
            row.try_get(20).unwrap_or(0.0),
            row.try_get(21).unwrap_or(0.0),
            row.try_get(22).unwrap_or(0.0),
            row.try_get(23).unwrap_or(0.0),
            row.try_get(24).unwrap_or(0.0),
            row.try_get(25).unwrap_or(0.0),
            row.try_get(26).unwrap_or(0.0),
            row.try_get(27).unwrap_or(0.0),
            row.try_get(28).unwrap_or(0.0),
        );

        let vitamin_content = Vitamins::new(fat_sol, water_sol);

        let vitamin_option = match vitamin_content.is_zero() {
            true => Option::None,
            false => Some(vitamin_content),
        };

        Product::new(
            row.get(0),
            row.get(1),
            row.get(2),
            energy,
            carbs,
            fat,
            protein,
            salt,
            vitamin_option,
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

    let product_id = result.last_insert_rowid();

    match product.vitamins() {
        Some(v) => {
            sqlx::query(
                r#"
            INSERT INTO "Vitamins"
            ("product", "a", "d", "e", "k", "b1", "b2", "b3", "b5", "b6", "b7", "b9", "b12", "c")
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);
            "#,
            )
            .bind(product_id)
            .bind(v.a())
            .bind(v.d())
            .bind(v.e())
            .bind(v.k())
            .bind(v.b1())
            .bind(v.b2())
            .bind(v.b3())
            .bind(v.b5())
            .bind(v.b6())
            .bind(v.b7())
            .bind(v.b9())
            .bind(v.b12())
            .bind(v.c())
            .execute(&mut tx)
            .await?;
        }
        None => {}
    }

    tx.commit().await?;

    Ok(product_id)
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
