mod base;
mod vitamins;

use std::ops::Add;

pub use self::base::Carbohydrates;
pub use self::base::Energy;
pub use self::base::Fat;
pub use self::base::MonoUnsaturatedFat;
pub use self::base::Nutrients;
pub use self::base::PolyUnsaturatedFat;
pub use self::base::Protein;
pub use self::base::Salt;
pub use self::base::TotalAble;
pub use self::base::UnsaturatedFat;

pub use self::vitamins::Minerals;
pub use self::vitamins::Vitamins;
pub use self::vitamins::{FatSoluble, FatSolubleApi};
pub use self::vitamins::{WaterSoluble, WaterSolubleApi};

fn add_options<T: Add<Output = T> + Clone>(a: &Option<T>, b: &Option<T>) -> Option<T> {
    if a.is_some() && b.is_none() {
        return a.clone().take();
    }

    if a.is_none() && b.is_some() {
        return b.clone().take();
    }

    if a.is_some() && b.is_some() {
        return Some(a.clone().unwrap() + b.clone().unwrap());
    }

    Option::None
}
