//  check for table 
// SELECT name FROM sqlite_master WHERE type='table' AND name='Food';

// Get column metadata 
// PRAGMA table_info('Food');

use sqlx::sqlite::SqliteRow;
use sqlx::SqlitePool;
use sqlx::row::Row;


struct TableMeta {
    pub name: String
}

pub async fn setup(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    match db_existance_check(pool).await {
        Ok(_v) => {
            return Ok(true);
        },
        Err(_err) => {
            // warn!("{:?}", "Food table doesn't exist");
            println!("Table doesnt exist");
            create_food_table(pool).await?;
        }
    }

    Ok(true)
}

async fn db_existance_check(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let _result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='Food'")
        .map(|row: SqliteRow| {
            TableMeta {
                name: row.get(0),
            }
        })
        .fetch_one(pool)
        .await?;

    Ok(())
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
    ).execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}