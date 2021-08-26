use sqlx::SqlitePool;

pub async fn setup(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS "Food" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "name"	TEXT NOT NULL,
            "manufacturer"	TEXT,
            "kcal"	REAL NOT NULL DEFAULT 0,
            "kj"	REAL NOT NULL DEFAULT 0,
            "carbohydrates"	REAL NOT NULL DEFAULT 0,
            "fiber"	REAL DEFAULT 0,
            "sugar"	REAL DEFAULT 0,
            "added_sugar"	REAL DEFAULT 0,
            "starch"	REAL DEFAULT 0,
            "fat"	REAL NOT NULL DEFAULT 0,
            "saturated"	REAL DEFAULT 0,
            "monounsaturated"	REAL DEFAULT 0,
            "trans"	REAL DEFAULT 0,
            "protein"	REAL NOT NULL DEFAULT 0,
            "salt"	REAL NOT NULL DEFAULT 0,
            "omegathree" REAL NOT NULL DEFAULT 0,
	        "omegasix" REAL NOT NULL DEFAULT 0
        );
        
        CREATE TABLE IF NOT EXISTS "Portions" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "product"	INTEGER NOT NULL,
            "name"	TEXT NOT NULL,
            "grams"	REAL NOt NULL,
            FOREIGN KEY("product") REFERENCES "Food"("id") ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS "Body" (
            "date"	TEXT NOT NULL UNIQUE,
            "mass"	REAL,
            "fat"	REAL,
            PRIMARY KEY("date")
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
            FOREIGN KEY("product") REFERENCES "Food"("id") ON DELETE CASCADE
        );
        
        CREATE VIEW IF NOT EXISTS full_product
        AS 
           SELECT f.id, f.name, f.manufacturer, f.kcal, f.kj, f.carbohydrates, f.fiber, f.sugar, f.added_sugar, f.starch, f.fat, f.saturated, f.monounsaturated, f.trans, f.protein, f.salt, v.a, v.d, v.e, v.k, v.b1, v.b2, v.b3, v.b5, v.b6, v.b7, v.b9, v.b12, v.c, f.omegathree, f.omegasix FROM Food as f LEFT JOIN Vitamins as v ON f.id = v.product;
        "#
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(true)
}
