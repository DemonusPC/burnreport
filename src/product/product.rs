use crate::nutrients::{
    Carbohydrates, Energy, Fat, FatSoluble, FatSolubleApi, MonoUnsaturatedFat, Nutrients,
    PolyUnsaturatedFat, Protein, Salt, TotalAble, UnsaturatedFat, Vitamins, WaterSoluble,
    WaterSolubleApi,
};
use crate::spi::StandardProductIdentifier;
use serde_derive::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub enum Unit {
    Grams,
    Mililiters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub nutrients: Nutrients,
    pub unit: Unit,
    pub spi: Option<StandardProductIdentifier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: i32,
    name: String,
    nutrients: Nutrients,
    unit: Unit,
    spi: Option<StandardProductIdentifier>,
}

impl Product {
    pub fn new(id: i32, name: String, nutrition: Nutrients, unit: Unit) -> Self {
        Self {
            id,
            name,
            nutrients: nutrition,
            unit,
            spi: None,
        }
    }

    pub fn new_with_spi(
        id: i32,
        name: String,
        nutrition: Nutrients,
        unit: Unit,
        spi: Option<StandardProductIdentifier>,
    ) -> Self {
        Self {
            id,
            name,
            nutrients: nutrition,
            unit,
            spi,
        }
    }

    pub fn from_row(row: &SqliteRow) -> Self {
        let energy: Energy = Energy::new(row.get("kcal"), row.get("kj"));
        let carbs: Carbohydrates = Carbohydrates::new(row.get("carbohydrates"), row.get("sugar"))
            .with_fiber(row.try_get("fiber").unwrap_or_default())
            .with_added_sugar(row.try_get("added_sugar").unwrap_or_default())
            .with_starch(row.try_get("starch").unwrap_or_default())
            .build();

        let monounsaturated = match row.try_get("monounsaturated") {
            Ok(v) => match v > 0.0 {
                false => Option::None,
                true => Some(MonoUnsaturatedFat::new(
                    v,
                    row.try_get("omega_7").unwrap_or_default(),
                    row.try_get("omega_9").unwrap_or_default(),
                )),
            },
            Err(_error) => Option::None,
        };
        let polysaturated = match row.try_get("polyunsaturated") {
            Ok(v) => match v > 0.0 {
                false => Option::None,
                true => Some(PolyUnsaturatedFat::new(
                    v,
                    row.try_get("omega_3").unwrap_or_default(),
                    row.try_get("omega_6").unwrap_or_default(),
                )),
            },
            Err(_error) => Option::None,
        };

        let unsaturated = match monounsaturated.is_some() || polysaturated.is_some() {
            true => Some(UnsaturatedFat::new(monounsaturated, polysaturated)),
            false => Option::None,
        };
        let fat: Fat = Fat::new(row.get("fat"), row.get("saturated"))
            .with_unsaturated(unsaturated)
            .with_trans(row.try_get("trans").unwrap_or_default())
            .build();

        let protein: Protein = Protein::new(row.get("protein"));
        let salt: Salt = Salt::new(row.get("salt"));

        let fat_sol = FatSoluble::new(
            row.try_get("a").unwrap_or_default(),
            row.try_get("d").unwrap_or_default(),
            row.try_get("e").unwrap_or_default(),
            row.try_get("k").unwrap_or_default(),
        );
        let water_sol = WaterSoluble::new(
            row.try_get("b1").unwrap_or_default(),
            row.try_get("b2").unwrap_or_default(),
            row.try_get("b3").unwrap_or_default(),
            row.try_get("b5").unwrap_or_default(),
            row.try_get("b6").unwrap_or_default(),
            row.try_get("b7").unwrap_or_default(),
            row.try_get("b9").unwrap_or_default(),
            row.try_get("b12").unwrap_or_default(),
            row.try_get("c").unwrap_or_default(),
        );

        let vitamin_content = Vitamins::new(fat_sol, water_sol);

        let vitamin_option = match vitamin_content.is_zero() {
            true => Option::None,
            false => Some(vitamin_content),
        };

        let nutrition: Nutrients =
            Nutrients::new(energy, carbs, fat, protein, salt, vitamin_option);

        let unit = match row.get(2) {
            "ml" => Unit::Mililiters,
            _ => Unit::Grams,
        };

        let product_spi = match row.try_get("numeric_code") {
            Ok(numeric_code) => Some(StandardProductIdentifier::new(
                numeric_code,
                row.get("alphabetic_code"),
                row.get("full_name"),
            )),
            Err(_e) => None,
        };

        Product::new_with_spi(row.get("id"), row.get("name"), nutrition, unit, product_spi)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients.clone()
    }

    pub fn energy(&self) -> &Energy {
        self.nutrients.energy()
    }

    pub fn carbohydrates(&self) -> &Carbohydrates {
        self.nutrients.carbohydrates()
    }

    pub fn fat(&self) -> &Fat {
        self.nutrients.fat()
    }

    pub fn protein(&self) -> &Protein {
        self.nutrients.protein()
    }

    pub fn salt(&self) -> &Salt {
        self.nutrients.salt()
    }

    pub fn vitamins(&self) -> Option<Vitamins> {
        self.nutrients.vitamins()
    }

    pub fn spi(&self) -> Option<StandardProductIdentifier> {
        self.spi.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlatProduct {
    // pub id: i32,
    pub name: String,
    pub unit: String,
    pub kj: f64,
    pub kcal: f64,
    pub carbohydrates: f64,
    pub sugar: f64,
    pub fiber: Option<f64>,
    pub added_sugar: Option<f64>,
    pub starch: Option<f64>,
    pub fat: f64,
    pub saturated: f64,
    pub monounsaturated: Option<f64>,
    pub omega_7: Option<f64>,
    pub omega_9: Option<f64>,
    pub polyunsaturated: Option<f64>,
    pub omega_3: Option<f64>,
    pub omega_6: Option<f64>,
    pub trans: f64,
    pub protein: f64,
    pub salt: f64,
    // vitamins
    pub a: Option<f64>,
    pub d: Option<f64>,
    pub e: Option<f64>,
    pub k: Option<f64>,
    pub b1: Option<f64>,
    pub b2: Option<f64>,
    pub b3: Option<f64>,
    pub b5: Option<f64>,
    pub b6: Option<f64>,
    pub b7: Option<f64>,
    pub b9: Option<f64>,
    pub b12: Option<f64>,
    pub c: Option<f64>,
}

pub struct ProductStore {}

impl ProductStore {
    pub async fn import_file(
        pool: &SqlitePool,
        products: &[FlatProduct],
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        for p in products {
            let unit = match p.unit.as_str() {
                "Grams" => "Grams",
                "Mililiters" => "Mililiters",
                _ => "Grams",
            };

            let result = sqlx::query(r#"INSERT INTO Products ("name", "unit", "kj", "kcal", "carbohydrates", "sugar", "fiber", "added_sugar", "starch", "fat", "saturated", "monounsaturated", "omega_7", "omega_9", "polyunsaturated", "omega_3", "omega_6", "trans", "protein", "salt")  
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20) "#)
            .bind(p.name.as_str())
            .bind(unit)
            .bind(p.kj)
            .bind(p.kcal)
            .bind(p.carbohydrates)
            .bind(p.sugar)
            .bind(p.fiber)
            .bind(p.added_sugar)
            .bind(p.starch)
            .bind(p.fat)
            .bind(p.saturated)
            .bind(p.monounsaturated)
            .bind(p.omega_7)
            .bind(p.omega_9)
            .bind(p.polyunsaturated)
            .bind(p.omega_3)
            .bind(p.omega_6)
            .bind(p.trans)
            .bind(p.protein)
            .bind(p.salt)
            .execute(&mut tx)
            .await?;

            let product_id = result.last_insert_rowid();

            sqlx::query(
                r#"
            INSERT INTO "Vitamins"
            ("product", "a", "d", "e", "k", "b1", "b2", "b3", "b5", "b6", "b7", "b9", "b12", "c")
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);
            "#,
            )
            .bind(product_id)
            .bind(p.a)
            .bind(p.d)
            .bind(p.e)
            .bind(p.k)
            .bind(p.b1)
            .bind(p.b2)
            .bind(p.b3)
            .bind(p.b5)
            .bind(p.b6)
            .bind(p.b7)
            .bind(p.b9)
            .bind(p.b12)
            .bind(p.c)
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn export_file(pool: &SqlitePool) -> Result<Vec<FlatProduct>, sqlx::Error> {
        let result = sqlx::query("SELECT * FROM full_product")
            .map(|row: SqliteRow| {
                let unit = match row.get(2) {
                    "Grams" => "Grams",
                    "Mililiters" => "Mililiters",
                    _ => "Grams",
                };
                FlatProduct {
                    name: row.get(1),
                    unit: unit.to_string(),
                    kj: row.get(3),
                    kcal: row.get(4),
                    carbohydrates: row.get(5),
                    sugar: row.get(6),
                    fiber: row.try_get(7).unwrap_or_default(),
                    added_sugar: row.try_get(8).unwrap_or_default(),
                    starch: row.try_get(9).unwrap_or_default(),
                    fat: row.get(10),
                    saturated: row.get(11),
                    monounsaturated: row.try_get(12).unwrap_or_default(),
                    omega_7: row.try_get(13).unwrap_or_default(),
                    omega_9: row.try_get(14).unwrap_or_default(),
                    polyunsaturated: row.try_get(15).unwrap_or_default(),
                    omega_3: row.try_get(16).unwrap_or_default(),
                    omega_6: row.try_get(17).unwrap_or_default(),
                    trans: row.try_get(18).unwrap_or_default(),
                    protein: row.get(19),
                    salt: row.get(20),
                    a: row.try_get(21).unwrap_or_default(),
                    d: row.try_get(22).unwrap_or_default(),
                    e: row.try_get(23).unwrap_or_default(),
                    k: row.try_get(24).unwrap_or_default(),
                    b1: row.try_get(25).unwrap_or_default(),
                    b2: row.try_get(26).unwrap_or_default(),
                    b3: row.try_get(27).unwrap_or_default(),
                    b5: row.try_get(28).unwrap_or_default(),
                    b6: row.try_get(29).unwrap_or_default(),
                    b7: row.try_get(30).unwrap_or_default(),
                    b9: row.try_get(31).unwrap_or_default(),
                    b12: row.try_get(32).unwrap_or_default(),
                    c: row.try_get(33).unwrap_or_default(),
                }
            })
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn single_product(pool: &SqlitePool, id: i32) -> Result<Product, sqlx::Error> {
        let result = sqlx::query("SELECT * FROM full_product WHERE id = ?")
            .bind(id)
            .map(|row: SqliteRow| {
                return Product::from_row(&row);
            })
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

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
                    (c/100) * $1 as c
                    FROM full_product WHERE id = $2"#,
        )
        .bind(amount)
        .bind(id)
        .map(|row: SqliteRow| Product::from_row(&row))
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn amount_adjusted_product_v2(
        pool: &SqlitePool,
        id: i64,
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
                    (c/100) * $1 as c
                    FROM full_product WHERE id = $2"#,
        )
        .bind(amount)
        .bind(id)
        .map(|row: SqliteRow| Product::from_row(&row))
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn amount_adjusted_spi_products_v2(
        pool: &SqlitePool,
        id: i64,
        amount: f64,
    ) -> Result<Vec<Product>, sqlx::Error> {
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
                    s.numeric
                    FROM full_product LEFT JOIN SPI as s ON s.numeric_code = spi WHERE spi = $2"#,
        )
        .bind(amount)
        .bind(id)
        .map(|row: SqliteRow| Product::from_row(&row))
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    pub async fn insert_product(
        pool: &SqlitePool,
        product: CreateProductRequest,
    ) -> Result<i64, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let raw_unit = match product.unit {
            Unit::Grams => "Grams".to_owned(),
            Unit::Mililiters => "Mililiters".to_owned(),
        };

        let monounsaturated = match product.nutrients.fat().unsaturated() {
            Some(v) => v.mono(),
            None => Option::None,
        };

        let (mono_total, omega_7, omega_9) = match monounsaturated {
            Some(v) => (v.total(), v.omega_7(), v.omega_9()),
            None => (0.0, Option::None, Option::None),
        };

        let polyunsaturated = match product.nutrients.fat().unsaturated() {
            Some(v) => v.poly(),
            None => Option::None,
        };

        let (poly_total, omega_3, omega_6) = match polyunsaturated {
            Some(v) => (v.total(), v.omega_3(), v.omega_6()),
            None => (0.0, Option::None, Option::None),
        };

        let spi = match product.spi {
            Some(v) => Some(v.numeric_code()),
            None => None,
        };

        let result = sqlx::query(r#"INSERT INTO Products ("name", "unit", "kj", "kcal", "carbohydrates", "sugar", "fiber", "added_sugar", "starch", "fat", "saturated", "monounsaturated", "omega_7", "omega_9", "polyunsaturated", "omega_3", "omega_6", "trans", "protein", "salt", "spi")  
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21) "#)
        .bind(product.name)
        .bind(raw_unit)
        .bind(product.nutrients.energy().k_j())
        .bind(product.nutrients.energy().kcal())
        .bind(product.nutrients.carbohydrates().total())
        .bind(product.nutrients.carbohydrates().sugar())
        .bind(product.nutrients.carbohydrates().fiber())
        .bind(product.nutrients.carbohydrates().added_sugar())
        .bind(product.nutrients.carbohydrates().starch())
        .bind(product.nutrients.fat().total())
        .bind(product.nutrients.fat().saturated())
        .bind(mono_total)
        .bind(omega_7)
        .bind(omega_9)
        .bind(poly_total)
        .bind(omega_3)
        .bind(omega_6)
        .bind(product.nutrients.fat().trans())
        .bind(product.nutrients.protein().total())
        .bind(product.nutrients.salt().total())
        .bind(spi)
        .execute(&mut tx)
        .await?;

        let product_id = result.last_insert_rowid();

        if let Some(v) = product.nutrients.vitamins() {
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

    pub async fn list_by_spi(
        pool: &SqlitePool,
        numeric_code: i64,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let result = sqlx::query(
            "SELECT * FROM full_product LEFT JOIN SPI as s ON s.numeric_code = spi WHERE spi = ?1;",
        )
        .bind(numeric_code)
        .map(|row: SqliteRow| {
            return Product::from_row(&row);
        })
        .fetch_all(pool)
        .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::{
        config::setup,
        nutrients::Nutrients,
        product::{product::CreateProductRequest, Unit},
        spi::{StandardProductIdentifier, StandardProductIdentifierStore},
    };

    use super::ProductStore;

    #[actix_web::test]
    async fn get_products_by_spi_numeric_code() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let spi = StandardProductIdentifier::new(420, "UNBTTR", "Unsalted Butter");

        StandardProductIdentifierStore::save(&pool, &spi)
            .await
            .unwrap();

        let expected_name = "Walmart Unsalted Butter";

        let product = CreateProductRequest {
            name: expected_name.to_string(),
            nutrients: Nutrients::default(),
            unit: Unit::Grams,
            spi: Some(spi),
        };

        ProductStore::insert_product(&pool, product).await.unwrap();

        let result = ProductStore::list_by_spi(&pool, 420).await.unwrap();

        assert_eq!(result.len(), 1);

        let result_product = &result[0];
        assert_eq!(result_product.name(), expected_name)
    }
}
