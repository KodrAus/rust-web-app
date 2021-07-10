/*! Commands for modifying product state. */

mod create_product;
mod set_product_title;

pub use self::{
    create_product::*,
    set_product_title::*,
};
