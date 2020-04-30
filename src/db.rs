use crate::nutrients::{Carbohydrates, Energy, Fat, Protein, Salt};

use crate::products::Product;

use sqlx::row::Row;
use sqlx::sqlite::SqliteConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::Connect;

use crate::file::import_products;
use crate::nutrients::TotalAble;
use sqlx::SqlitePool;

pub async fn import_file(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let test = import_products("./data/products.csv");
    let result = test.unwrap();

    println!("{:?}", result);

    for prod in result {
        sqlx::query!(
            r#"
    INSERT INTO Food ( name, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt)
    VALUES ( $1, $2, $3, $4,$5, $6,$7, $8,$9, $10,$11, $12,$13, $14,$15 )
            "#,
            prod.name(),
            prod.manufacturer(),
            prod.energy().kcal(),
            prod.energy().k_j(),
            prod.carbohydrates().total(),
            prod.carbohydrates().fiber(),
            prod.carbohydrates().sugar(),
            prod.carbohydrates().added_sugar(),
            prod.carbohydrates().starch(),
            prod.fat().total(),
            prod.fat().saturated(),
            prod.fat().monounsaturated(),
            prod.fat().trans(),
            prod.protein().total(),
            prod.salt().total()

        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn search_products(pool: &SqlitePool, term: &str) -> Result<Vec<Product>, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT * FROM Food WHERE name LIKE $1")
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

pub async fn single_product(pool: &SqlitePool, id: i32) -> Result<Vec<Product>, sqlx::Error> {
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
        .fetch_all(pool)
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
