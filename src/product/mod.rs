mod portion;
mod product;
mod search;

pub use self::product::amount_adjusted_product;
pub use self::product::delete_product;
pub use self::product::export_file;
pub use self::product::import_file;
pub use self::product::insert_product;
pub use self::product::single_product;
pub use self::product::FlatProduct;
pub use self::product::Product;
pub use self::product::Unit;

pub use self::portion::PortionStore;
pub use self::portion::Portion;
pub use self::search::search_product_suggestions;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultList<T> {
    pub result: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    code: u16,
    status: Option<String>,
    id: Option<i64>,
}

impl ApiResult {
    pub fn new(code: u16, status: Option<String>, id: Option<i64>) -> Self {
        ApiResult { code, status, id }
    }
}
