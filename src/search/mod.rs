use serde_derive::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SearchEntity {
    Product,
    Recipie,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestion {
    id: i64,
    text: String,
    sub_text: Option<String>,
    entity: SearchEntity,
}

impl SearchSuggestion {
    pub fn new(id: i64, text: String, sub_text: Option<String>, entity: SearchEntity) -> Self {
        SearchSuggestion {
            id,
            text,
            sub_text,
            entity,
        }
    }
}

impl PartialEq for SearchSuggestion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.text == other.text
            && self.sub_text == other.sub_text
            && self.entity == other.entity
    }
}

pub struct SearchStore {}

impl SearchStore {
    pub async fn search(
        pool: &SqlitePool,
        term: &str,
        entity: Option<SearchEntity>,
    ) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
        todo!("Not implemented")
    }
}

// pub async fn search_product_suggestions(
//     pool: &SqlitePool,
//     term: &str,
// ) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
//     // SELECT *  FROM Food WHERE name LIKE "%Spag%";
//     let result = sqlx::query("SELECT id, name, unit FROM Products WHERE name LIKE $1 LIMIT 15")
//         .bind(format!("%{}%", term))
//         .map(|row: SqliteRow| {
//             let id: i32 = row.get(0);
//             let text: String = row.get(1);
//             let sub_text: String = row.get(2);
//             SearchSuggestion::new(id, text, Some(sub_text), Some("Product".to_owned()))
//         })
//         .fetch_all(pool)
//         .await?;
//     Ok(result)
// }

// pub async fn search_recipie_suggestions(pool: &SqlitePool, term: &str) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
//     let result = sqlx::query("SELECT id, name FROM Recipies WHERE name LIKE $1 LIMIT 15")
//         .bind(format!("%{}%", term))
//         .map(|row: SqliteRow| {
//             let id: i32 = row.get(0);
//             let text: String = row.get(1);
//             SearchSuggestion::new(id, text, None, Some("Recipie".to_owned()))
//         })
//         .fetch_all(pool)
//         .await?;
//     Ok(result)
// }
#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::{
        config::setup,
        nutrients::Nutrients,
        product::{Product, ProductStore, Unit},
        recipie::{IngredientCreateCommand, RecipieCreateCommand, RecipieStore},
    };

    use super::{SearchEntity, SearchStore, SearchSuggestion};

    #[actix_web::test]
    async fn can_search_for_product_and_recipies() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        // Basic setup
        let ingredient_one = Product::new(
            0,
            "Test Product".to_owned(),
            Nutrients::default(),
            Unit::Grams,
        );
        let ingredient_id_one = ProductStore::insert_product(&pool, ingredient_one)
            .await
            .unwrap();

        let ingredients: Vec<IngredientCreateCommand> = vec![IngredientCreateCommand {
            amount: 20.0,
            product_id: ingredient_id_one,
        }];

        let recipie = RecipieCreateCommand {
            name: "Test Recipie".to_owned(),
            ingredients,
        };

        let recipie_id = RecipieStore::create(&pool, recipie).await.unwrap();

        let result = SearchStore::search(&pool, "Test", None).await.unwrap();

        assert_eq!(result.len(), 2);

        let product_search_suggestion = SearchSuggestion::new(
            ingredient_id_one,
            "Test Product".to_owned(),
            None,
            SearchEntity::Product,
        );
        let recipie_search_suggestion = SearchSuggestion::new(
            recipie_id,
            "Test Recipie".to_owned(),
            None,
            SearchEntity::Recipie,
        );
        assert_eq!(result.contains(&product_search_suggestion), true);
        assert_eq!(result.contains(&recipie_search_suggestion), true);
    }
}
