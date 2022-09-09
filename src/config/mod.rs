use sqlx::SqlitePool;

pub async fn setup(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS "Products" (
            "id"	INTEGER NOT NULL,
            "name"	TEXT NOT NULL,
            "unit"	TEXT NOT NULL,
            "kj"	REAL NOT NULL,
            "kcal"	REAL NOT NULL,
            "carbohydrates"	REAL NOT NULL,
            "sugar"	REAL NOT NULL,
            "fiber"	REAL,
            "added_sugar"	REAL,
            "starch"	REAL,
            "fat"	REAL NOT NULL,
            "saturated"	REAL NOT NULL,
            "monounsaturated"	REAL,
            "omega_7"	REAL,
            "omega_9"	REAL,
            "polyunsaturated"	REAL,
            "omega_3"	REAL,
            "omega_6"	REAL,
            "trans"	REAL,
            "protein"	REAL NOT NULL,
            "salt"	REAL NOT NULL,
            PRIMARY KEY("id" AUTOINCREMENT)
        );        

        CREATE TABLE IF NOT EXISTS "Vitamins" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "product"	INTEGER NOT NULL,
            "a"	REAL,
            "d"	REAL,
            "e"	REAL,
            "k"	REAL,
            "b1"	REAL,
            "b2"	REAL,
            "b3"	REAL,
            "b5"	REAL,
            "b6"	REAL,
            "b7"	REAL,
            "b9"	REAL,
            "b12"	REAL,
            "c"	REAL,
            FOREIGN KEY("product") REFERENCES "Products"("id") ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS "Portions" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "product"	INTEGER NOT NULL,
            "name"	TEXT NOT NULL,
            "grams"	REAL NOt NULL,
            FOREIGN KEY("product") REFERENCES "Products"("id") ON DELETE CASCADE
        );
        CREATE VIEW IF NOT EXISTS full_product
            AS 
            SELECT f.id, f.name, f.unit, f.kj,  f.kcal, f.carbohydrates, f.sugar, f.fiber, f.added_sugar, f.starch, f.fat, f.saturated, f.monounsaturated, f.omega_7, f.omega_9, f.polyunsaturated, f.omega_3, f.omega_6, f.trans, f.protein, f.salt, v.a, v.d, v.e, v.k, v.b1, v.b2, v.b3, v.b5, v.b6, v.b7, v.b9, v.b12, v.c FROM Products as f LEFT JOIN Vitamins as v ON f.id = v.product;
        
        "#
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(true)
}
