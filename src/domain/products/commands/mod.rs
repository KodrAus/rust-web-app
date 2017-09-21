/*! Commands for modifying product state. */

pub mod create_product;
pub use self::create_product::{CreateProduct, CreateProductCommand};

pub mod set_product_title;
pub use self::set_product_title::{SetProductTitle, SetProductTitleCommand};
