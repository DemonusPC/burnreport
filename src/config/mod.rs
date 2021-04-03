//  check for table
// SELECT name FROM sqlite_master WHERE type='table' AND name='Food';

// Get column metadata
// PRAGMA table_info('Food');

use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use sqlx::SqlitePool;

struct TableMeta {
    pub name: String,
}

async fn check_if_table_exists(pool: &SqlitePool, table_name: &str) -> Result<(), sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let _result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name=$1")
        .bind(table_name)
        .map(|row: SqliteRow| TableMeta { name: row.get(0) })
        .fetch_one(pool)
        .await?;

    Ok(())
}

pub async fn setup(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    match check_if_table_exists(pool, "Food").await {
        Ok(_v) => println!("Food table exists"),
        Err(err) => {
            println!("Check table for Food failed with error: {}", err);
            create_food_table(pool).await?;
        }
    }

    match check_if_table_exists(pool, "Portions").await {
        Ok(_v) => println!("Portions table exists"),
        Err(err) => {
            println!("Check table for Portions failed with error: {}", err);
            create_portions_table(pool).await?;
        }
    }

    match check_if_table_exists(pool, "Body").await {
        Ok(_v) => println!("Body table exists"),
        Err(err) => {
            println!("Check table for Body failed with error: {}", err);
            create_body_table(pool).await?;
        }
    }

    Ok(true)
}

async fn create_food_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
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
            "salt"	REAL NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}

async fn create_portions_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS "Portions" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            "product"	INTEGER NOT NULL,
            "name"	TEXT NOT NULL,
            "grams"	REAL NOt NULL,
            FOREIGN KEY("product") REFERENCES "Food"("id") ON DELETE CASCADE
        );
        "#
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}

// I should refactor this to be a common function but im not bothered now
async fn create_body_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS "Body" (
            "date"	TEXT NOT NULL UNIQUE,
            "mass"	INTEGER,
            "fat"	INTEGER,
            PRIMARY KEY("date")
        )
        "#
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}
