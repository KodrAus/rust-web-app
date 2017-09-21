/*! Queries for fetching product state. */

pub mod get_product;
pub use self::get_product::{GetProduct, GetProductQuery};

pub mod get_product_summaries;
pub use self::get_product_summaries::{ProductSummary, GetProductSummaries, GetProductSummariesQuery};
