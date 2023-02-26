use std::{fmt::Display, str::FromStr};

use serde_derive::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SearchEntity {
    Product,
    Recipie,
    Spi,
}

impl FromStr for SearchEntity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Product" => Ok(SearchEntity::Product),
            "Recipie" => Ok(SearchEntity::Recipie),
            "Spi" => Ok(SearchEntity::Spi),
            _ => Err(()),
        }
    }
}

impl Display for SearchEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchEntity::Product => write!(f, "Product"),
            SearchEntity::Recipie => write!(f, "Recipie"),
            SearchEntity::Spi => write!(f, "Spi"),
        }
    }
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
    ) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
        let result = sqlx::query("SELECT * FROM search WHERE name LIKE $1 LIMIT 16")
            .bind(format!("%{}%", term))
            .try_map(|row: SqliteRow| {
                let id: i64 = row.get("id");
                let text: String = row.get("name");
                let entity_string: String = row.get("entity");
                let entity: SearchEntity = match SearchEntity::from_str(&entity_string) {
                    Ok(v) => v,
                    // This error is not exactly correct since its a failure of decoding
                    // However we should never find any rows that cant be parsed here
                    Err(_e) => return Err(sqlx::Error::RowNotFound),
                };
                Ok(SearchSuggestion::new(id, text, None, entity))
            })
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn search_by_entity(
        pool: &SqlitePool,
        term: &str,
        entity: SearchEntity,
    ) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
        let result =
            sqlx::query("SELECT * FROM search WHERE name LIKE $1 AND entity = $2 LIMIT 16")
                .bind(format!("%{}%", term))
                .bind(entity.to_string())
                .try_map(|row: SqliteRow| {
                    let id: i64 = row.get("id");
                    let text: String = row.get("name");
                    let entity_string: String = row.get("entity");
                    let entity: SearchEntity = match SearchEntity::from_str(&entity_string) {
                        Ok(v) => v,
                        // This error is not exactly correct since its a failure of decoding
                        // However we should never find any rows that cant be parsed here
                        Err(_e) => return Err(sqlx::Error::RowNotFound),
                    };
                    Ok(SearchSuggestion::new(id, text, None, entity))
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
        product::{CreateProductRequest,  ProductStore, Unit},
        recipie::{IngredientCreateCommand, RecipieCreateCommand, RecipieStore},
    };

    use super::{SearchEntity, SearchStore, SearchSuggestion};

    #[actix_web::test]
    async fn can_search_for_product_and_recipies() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        setup(&pool).await.unwrap();

        // Basic setup
        let ingredient_one = CreateProductRequest {
            name: "Test Product".to_owned(),
            nutrients: Nutrients::default(),
            unit: Unit::Grams,
            spi: None,
        };
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

        let result = SearchStore::search(&pool, "Test").await.unwrap();

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

        let specific_result = SearchStore::search_by_entity(&pool, "Test", SearchEntity::Product)
            .await
            .unwrap();

        assert_eq!(specific_result.len(), 1);
        assert_eq!(specific_result.contains(&product_search_suggestion), true);
    }
}
