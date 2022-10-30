use serde_derive::{Deserialize, Serialize};
use sqlx::{SqlitePool, Sqlite};

use crate::{product::{Product, Unit, self}, nutrients::Nutrients};
#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    product: Product,
    amount: f64,
}

impl Ingredient {
    pub fn new(product: Product, amount: f64, ) -> Self {
        Self {
            product,
            amount,
        }
    }
    pub fn product(&self) -> &Product {
        &self.product
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipie {
    id: i64,
    name: String,
    ingredients: Vec<Ingredient>,
    total: Nutrients
}

impl Recipie {
    pub fn new(id: i64, name: String, ingredients: Vec<Ingredient>) -> Self {

        let total = ingredients.as_slice().into_iter().fold(Nutrients::empty(), |acc,x|{
            let add = acc + x.product().nutrients();
            add
        });


        Self {
            id,
            name,
            ingredients,
            total 
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Page {
    prev: Option<String>,
    next: Option<String>
}

impl Page {
    pub fn new(prev: Option<String>, next: Option<String>) -> Self {
        Self {
            prev,
            next
        }
    }
    pub fn prev(&self) -> Option<String>{
        self.prev.clone()
    }
    pub fn next(&self) -> Option<String>{
        self.next.clone()
    }
}

pub struct RecipieStore {}


impl RecipieStore {

    // get
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Recipie, sqlx::Error> {
        todo!()
    }
    // create
    pub async fn create(pool: &SqlitePool, recipie: Recipie) -> Result<i64, sqlx::Error> {
        todo!()
    }
    // modify
    pub async fn update(pool: &SqlitePool, recipie: Recipie) -> Result<(), sqlx::Error> {
        todo!()
    }
    // delete
    pub async fn delete(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
        todo!()
    }
    // list
    pub async fn list(pool: &SqlitePool, page_size: i32, offset: Option<String>) -> Result<Page, sqlx::Error> {
        todo!()
    }
}
#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;

    use crate::{config::setup, product::{ProductStore, Product, Unit}, nutrients::Nutrients};

    use super::{RecipieStore, Recipie, Ingredient};

    // When we have the ledger database setup
    #[actix_web::test]
    async fn can_perform_all_recipie_store_operations(){
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();        

        let ingredient_one = Product::new(0, "Ingredient One".to_owned(), Nutrients::default(), Unit::Grams);
        let ingredient_two = Product::new(0, "Ingredient Two".to_owned(), Nutrients::default(), Unit::Grams);

        let ingredient_id_one = ProductStore::insert_product(&pool, ingredient_one).await.unwrap();
        let ingredient_id_two= ProductStore::insert_product(&pool, ingredient_two).await.unwrap();

        // This cast is dangerous. Should switch everything to using i64
        let p_one = ProductStore::single_product(&pool, ingredient_id_one as i32).await.unwrap();
        let p_two = ProductStore::single_product(&pool, ingredient_id_two as i32).await.unwrap();

        let ingredients: Vec<Ingredient> = vec![Ingredient::new(p_one, 20.0), Ingredient::new(p_two, 158.5)];

        let recipie = Recipie::new(0, "Test Recipie".to_owned(), ingredients);

        // Create a recipie
        let recipie_id = RecipieStore::create(&pool, recipie).await.unwrap();

        // Get the recipie by id
        let recipie_from_store = RecipieStore::get_by_id(&pool, recipie_id).await.unwrap();

        assert_eq!(recipie_id, recipie_from_store.id());
        assert_eq!("Test Recipie", recipie_from_store.name());

        // Updating a recipie
        // Deleting a recipie


    }

}
