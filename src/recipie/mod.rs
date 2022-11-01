use serde_derive::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, Sqlite, SqlitePool};

use crate::{
    nutrients::Nutrients,
    product::{self, Product, Unit},
};
#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    product: Product,
    amount: f64,
}

impl Ingredient {
    pub fn new(product: Product, amount: f64) -> Self {
        Self { product, amount }
    }
    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipie {
    id: i64,
    name: String,
    ingredients: Vec<Ingredient>,
    total: Nutrients,
}

impl Recipie {
    pub fn new(id: i64, name: String, ingredients: Vec<Ingredient>) -> Self {
        let total = ingredients
            .as_slice()
            .into_iter()
            .fold(Nutrients::empty(), |acc, x| {
                let add = acc + x.product().nutrients();
                add
            });

        Self {
            id,
            name,
            ingredients,
            total,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ingredients(&self) -> &[Ingredient] {
        &self.ingredients
    }

    pub fn set_ingredients(&mut self, ingredients: Vec<Ingredient>) {
        self.ingredients = ingredients;
    }
}

pub struct Page {
    recipies: Vec<Recipie>,
    prev: Option<String>,
    next: Option<String>,
}

impl Page {
    pub fn new(recipies: Vec<Recipie>, prev: Option<String>, next: Option<String>) -> Self {
        Self { recipies, prev, next }
    }
    pub fn recipies(&self) -> &[Recipie] {
        &self.recipies
    }

    pub fn prev(&self) -> Option<String> {
        self.prev.clone()
    }

    pub fn next(&self) -> Option<String> {
        self.next.clone()
    }
}

pub struct RecipieStore {}

impl RecipieStore {
    // get
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Recipie, sqlx::Error> {
        let ingredients = sqlx::query("SELECT Ingredients.amount, full_product.* FROM Ingredients LEFT JOIN full_product ON Ingredients.product_id = full_product.id WHERE Ingredients.recipie_id = ?1;").bind(id).map(|row: SqliteRow| {
            Ingredient::new(Product::from_row(&row), row.get("amount"))
        }).fetch_all(pool).await?;

        let mut result = sqlx::query("SELECT id, name FROM Recipies WHERE id = ?")
            .bind(id)
            .map(|row: SqliteRow| Recipie::new(row.get("id"), row.get("name"), vec![]))
            .fetch_one(pool)
            .await?;

        result.set_ingredients(ingredients);
        Ok(result)
    }
    // create
    pub async fn create(pool: &SqlitePool, recipie: Recipie) -> Result<i64, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let result = sqlx::query(r#"INSERT INTO Recipies ("name") VALUES (?1); "#)
            .bind(recipie.name().to_owned())
            .execute(&mut tx)
            .await?;

        let recipie_id = result.last_insert_rowid();

        for ingredient in recipie.ingredients() {
            sqlx::query(
                r#"
            INSERT INTO "Ingredients" ("amount", "recipie_id", "product_id") VALUES (?1, ?2, ?3); 
        "#,
            )
            .bind(ingredient.amount())
            .bind(recipie_id)
            .bind(ingredient.product().id())
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;
        Ok(recipie_id)
    }
    // modify
    pub async fn update(pool: &SqlitePool, recipie: Recipie) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("DELETE FROM Ingredients WHERE recipie_id = ?1")
            .bind(recipie.id())
            .execute(&mut tx)
            .await?;

        for ingredient in recipie.ingredients() {
            sqlx::query(
                    r#"
                INSERT INTO "Ingredients" ("amount", "recipie_id", "product_id") VALUES (?1, ?2, ?3); 
            "#,
                )
                .bind(ingredient.amount())
                .bind(recipie.id())
                .bind(ingredient.product().id())
                .execute(&mut tx)
                .await?;
        }

        sqlx::query(r#"UPDATE Recipies SET name= ?1 WHERE id = ?2;"#)
            .bind(recipie.name())
            .bind(recipie.id())
            .execute(&mut tx)
            .await?;

        tx.commit().await
    }
    // delete
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("DELETE FROM Recipies WHERE id = ?1").bind(id).execute(&mut tx).await?;

        tx.commit().await
    }
    // list
    pub async fn list(
        pool: &SqlitePool,
        page_size: i32,
        cursor: Option<String>,
    ) -> Result<Page, sqlx::Error> {

        let mut result = sqlx::query("SELECT id, name FROM Recipies")
        .map(|row: SqliteRow| Recipie::new(row.get("id"), row.get("name"), vec![]))
        .fetch_all(pool)
        .await?;

        Ok(Page::new(result,None, None))
    }
}
#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;

    use crate::{
        config::setup,
        nutrients::Nutrients,
        product::{Product, ProductStore, Unit},
    };

    use super::{Ingredient, Recipie, RecipieStore};

    // When we have the ledger database setup
    #[actix_web::test]
    async fn can_perform_all_recipie_store_operations() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        let ingredient_one = Product::new(
            0,
            "Ingredient One".to_owned(),
            Nutrients::default(),
            Unit::Grams,
        );
        let ingredient_two = Product::new(
            1,
            "Ingredient Two".to_owned(),
            Nutrients::default(),
            Unit::Grams,
        );

        let ingredient_id_one = ProductStore::insert_product(&pool, ingredient_one)
            .await
            .unwrap();
        let ingredient_id_two = ProductStore::insert_product(&pool, ingredient_two)
            .await
            .unwrap();

        // This cast is dangerous. Should switch everything to using i64
        let p_one = ProductStore::single_product(&pool, ingredient_id_one as i32)
            .await
            .unwrap();
        let p_two = ProductStore::single_product(&pool, ingredient_id_two as i32)
            .await
            .unwrap();

        let ingredients: Vec<Ingredient> =
            vec![Ingredient::new(p_one, 20.0), Ingredient::new(p_two, 158.5)];

        let recipie = Recipie::new(0, "Test Recipie".to_owned(), ingredients);

        // Create a recipie
        let recipie_id = RecipieStore::create(&pool, recipie).await.unwrap();
        // Get the recipie by id
        let recipie_from_store = RecipieStore::get_by_id(&pool, recipie_id).await.unwrap();

        assert_eq!(recipie_id, recipie_from_store.id());
        assert_eq!("Test Recipie", recipie_from_store.name());

        let recipie_ingredients = recipie_from_store.ingredients();

        let a = &recipie_ingredients[0];
        assert_eq!(a.amount(), 20.0);
        assert_eq!(a.product().name(), "Ingredient One");

        let b = &recipie_ingredients[1];
        assert_eq!(b.amount(), 158.5);
        assert_eq!(b.product().name(), "Ingredient Two");

        let ingredient_three = Product::new(
            1,
            "Ingredient Three".to_owned(),
            Nutrients::default(),
            Unit::Grams,
        );

        // Updating a recipie
        let updated_ingredients: Vec<Ingredient> = vec![Ingredient::new(ingredient_three, 20.0)];

        let updated_recipie =
            Recipie::new(recipie_id, "Updated Recipie".to_owned(), updated_ingredients);

        RecipieStore::update(&pool, updated_recipie).await.unwrap();


        let updated_result = RecipieStore::get_by_id(&pool, recipie_id).await.unwrap();
        assert_eq!(updated_result.ingredients().len(), 1);

        let c = &updated_result.ingredients()[0];

        assert_eq!(updated_result.name(), "Updated Recipie");
        assert_eq!(c.amount(), 20.0);
        assert_eq!(c.product().name(), "Ingredient One");

        // Deleting a recipie
        RecipieStore::delete(&pool, recipie_id).await.unwrap();

        let list_of_recipies = RecipieStore::list(&pool, 10, Option::None).await.unwrap();


        assert_eq!(list_of_recipies.recipies().len(), 0);

    }
}
