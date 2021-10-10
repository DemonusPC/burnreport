use crate::nutrients::FatSoluble;
use crate::nutrients::FatV2;
use crate::nutrients::MonoUnsaturatedFat;
use crate::nutrients::Nutrients;
use crate::nutrients::PolyUnsaturatedFat;
use crate::nutrients::TotalAble;
use crate::nutrients::UnsaturatedFat;
use crate::nutrients::WaterSoluble;
use crate::nutrients::{Carbohydrates, Energy, Protein, Salt};
use crate::nutrients::{FatSolubleApi, Vitamins, WaterSolubleApi};
use crate::products::{Portion, Product, SearchSuggestion, Unit};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::SqlitePool;

// pub async fn import_file(pool: &SqlitePool, products: &[Product]) -> Result<(), sqlx::Error> {
//     let mut tx = pool.begin().await?;
//     for product in products {
//         let result = sqlx::query(r#"INSERT INTO Food ( name, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt, omegathree, omegasix)
//         VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)"#)
//         .bind(product.name())
//         .bind(product.manufacturer())
//         .bind(product.energy().kcal())
//         .bind(product.energy().k_j())
//         .bind(product.carbohydrates().total())
//         .bind(product.carbohydrates().fiber())
//         .bind(product.carbohydrates().sugar())
//         .bind(product.carbohydrates().added_sugar())
//         .bind(product.carbohydrates().starch())
//         .bind(product.fat().total())
//         .bind(product.fat().saturated())
//         .bind(product.fat().monounsaturated())
//         .bind(product.fat().trans())
//         .bind(product.protein().total())
//         .bind(product.salt().total())
//         .bind(product.fat().omega_3())
//         .bind(product.fat().omega_6())
//         .execute(&mut tx)
//         .await?;
//     }

//     tx.commit().await?;

//     Ok(())
// }

pub async fn search_product_suggestions(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT id, name, unit FROM Products WHERE name LIKE $1 LIMIT 15")
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

pub async fn single_product(pool: &SqlitePool, id: i32) -> Result<Product, sqlx::Error> {
    let result = sqlx::query("SELECT * FROM Products WHERE id = ?")
        .bind(id)
        .map(|row: SqliteRow| {
            // let name : String = row.get(0);
            let energy: Energy = Energy::new(row.get(3), row.get(4));
            let carbs: Carbohydrates = Carbohydrates::new(
                row.get(5),
                row.get(6),
                row.try_get(7).unwrap_or(Option::None),
                row.try_get(8).unwrap_or(Option::None),
                row.get(9),
            );

            let monounsaturated = match row.try_get(12) {
                Ok(v) => Some(MonoUnsaturatedFat::new(
                    v,
                    row.try_get(13).unwrap_or(Option::None),
                    row.try_get(14).unwrap_or(Option::None),
                )),
                Err(_error) => Option::None,
            };
            let polysaturated = match row.try_get(15) {
                Ok(v) => Some(PolyUnsaturatedFat::new(
                    v,
                    row.try_get(16).unwrap_or(Option::None),
                    row.try_get(17).unwrap_or(Option::None),
                )),
                Err(_error) => Option::None,
            };

            let unsaturated = match monounsaturated.is_some() || polysaturated.is_some() {
                true => Some(UnsaturatedFat::new(monounsaturated, polysaturated)),
                false => Option::None,
            };
            let fat: FatV2 = FatV2::new(
                row.get(10),
                row.get(11),
                unsaturated,
                row.try_get(18).unwrap_or(Option::None),
            );

            let protein: Protein = Protein::new(row.get(19));
            let salt: Salt = Salt::new(row.get(20));

            let nutrition: Nutrients =
                Nutrients::new(energy, carbs, fat, protein, salt, Option::None);

            let unit = match row.get(2) {
                "ml" => Unit::Mililiters,
                _ => Unit::Grams,
            };
            Product::new(row.get(0), row.get(1), nutrition, unit)
        })
        .fetch_one(pool)
        .await?;
    Ok(result)
}

// SELECT id, manufacturer, (kcal/100) * 20 as kcal, (kj/100) * 20 as kj,  (carbohydrates/100) * 20 as carbohydrates,  (fiber/100) * 20 as fiber, (sugar/100) * 20 as sugar, (added_sugar/100) * 20 as added_sugar,  (starch/100) * 20 as starch, (fat/100) * 20 as fat, (saturated/100) * 20 as saturated, (monounsaturated/100) * 20 as monounsaturated, (trans/100) * 20 as trans, (protein/100) * 20 as protein, (salt/100) * 20 as salt FROM Food WHERE id = 1
pub async fn amount_adjusted_product(
    pool: &SqlitePool,
    id: i32,
    amount: f64,
) -> Result<Product, sqlx::Error> {
    let result = sqlx::query(
        r#"SELECT 
                                id, 
                                name, 
                                unit,
                                (kj/100) * $1 as kj,
                                (kcal/100) * $1 as kcal,
                                (carbohydrates/100) * $1 as carbohydrates,  
                                (sugar/100) * $1 as sugar, 
                                (fiber/100) * $1 as fiber, 
                                (added_sugar/100) * $1 as added_sugar,  
                                (starch/100) * $1 as starch, 
                                (fat/100) * $1 as fat, 
                                (saturated/100) * $1 as saturated, 
                                (monounsaturated/100) * $1 as monounsaturated, 
                                (omega_7/100) * $1 as omega_7, 
                                (omega_9/100) * $1 as omega_9, 
                                (polyunsaturated/100) * $1 as polyunsaturated, 
                                (omega_3/100) * $1 as omega_3, 
                                (omega_6/100) * $1 as omega_6, 
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
                                (c/100) * $1 as c,
                                FROM full_product WHERE id = $2"#,
    )
    .bind(amount)
    .bind(id)
    .map(|row: SqliteRow| {
        // let name : String = row.get(0);

        let energy: Energy = Energy::new(row.get(3), row.get(4));
        let carbs: Carbohydrates = Carbohydrates::new(
            row.get(5),
            row.get(6),
            row.try_get(7).unwrap_or(Option::None),
            row.try_get(8).unwrap_or(Option::None),
            row.get(9),
        );

        let monounsaturated = match row.try_get(12) {
            Ok(v) => Some(MonoUnsaturatedFat::new(
                v,
                row.try_get(13).unwrap_or(Option::None),
                row.try_get(14).unwrap_or(Option::None),
            )),
            Err(_error) => Option::None,
        };
        let polysaturated = match row.try_get(15) {
            Ok(v) => Some(PolyUnsaturatedFat::new(
                v,
                row.try_get(16).unwrap_or(Option::None),
                row.try_get(17).unwrap_or(Option::None),
            )),
            Err(_error) => Option::None,
        };

        let unsaturated = match monounsaturated.is_some() || polysaturated.is_some() {
            true => Some(UnsaturatedFat::new(monounsaturated, polysaturated)),
            false => Option::None,
        };
        let fat: FatV2 = FatV2::new(
            row.get(10),
            row.get(11),
            unsaturated,
            row.try_get(18).unwrap_or(Option::None),
        );

        let protein: Protein = Protein::new(row.get(19));
        let salt: Salt = Salt::new(row.get(20));

        let nutrition: Nutrients = Nutrients::new(energy, carbs, fat, protein, salt, Option::None);

        let unit = match row.get(2) {
            "ml" => Unit::Mililiters,
            _ => Unit::Grams,
        };

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

        let unit = match row.get(2) {
            "ml" => Unit::Mililiters,
            _ => Unit::Grams,
        };
        Product::new(row.get(0), row.get(1), nutrition, unit)
    })
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn insert_product(pool: &SqlitePool, product: Product) -> Result<i64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let raw_unit = match product.unit() {
        &Unit::Grams => "g".to_owned(),
        &Unit::Mililiters => "ml".to_owned(),
    };

    let monounsaturated = match product.fat().unsaturated() {
        Some(v) => v.mono(),
        None => Option::None,
    };

    let (mono_total, omega_7, omega_9) = match monounsaturated {
        Some(v) => (v.total(), v.omega_7(), v.omega_9()),
        None => (0.0, Option::None, Option::None),
    };

    let polyunsaturated = match product.fat().unsaturated() {
        Some(v) => v.poly(),
        None => Option::None,
    };

    let (poly_total, omega_3, omega_6) = match polyunsaturated {
        Some(v) => (v.total(), v.omega_3(), v.omega_3()),
        None => (0.0, Option::None, Option::None),
    };

    let result = sqlx::query(r#"INSERT INTO Products ("name", "unit", "kj", "kcal", "carbohydrates", "sugar", "fiber", "added_sugar", "starch", "fat", "saturated", "monounsaturated", "omega_7", "omega_9", "polyunsaturated", "omega_3", "omega_6", "trans", "protein", "salt")  
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20) "#)
    .bind(product.name())
    .bind(raw_unit)
    .bind(product.energy().k_j())
    .bind(product.energy().kcal())
    .bind(product.carbohydrates().total())
    .bind(product.carbohydrates().sugar())
    .bind(product.carbohydrates().fiber())
    .bind(product.carbohydrates().added_sugar())
    .bind(product.carbohydrates().starch())
    .bind(product.fat().total())
    .bind(product.fat().saturated())
    .bind(mono_total)
    .bind(omega_7)
    .bind(omega_9)
    .bind(poly_total)
    .bind(omega_3)
    .bind(omega_6)
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

    sqlx::query("DELETE FROM Products WHERE id = ?")
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

// Modify a product Size
// TODO: Add functionality for modifying product sizes
// Lol still didnt do that todo
