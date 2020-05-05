mod db;
mod handlers;
mod routes;

pub use self::routes::routes;
pub use self::routes::SearchQuery;

// SELECT id, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt FROM Food
