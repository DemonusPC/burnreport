mod db;
mod handlers;
mod routes;
mod error;

// pub use self::routes::routes;
pub use self::routes::SearchQuery;
pub use self::db::search_products;
pub use self::error::ApiError;

// SELECT id, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt FROM Food
