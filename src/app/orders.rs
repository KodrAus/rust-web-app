/*! `/orders` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{
        self,
        Error,
    },
    domain::{
        customers::*,
        infra::*,
        orders::*,
        products::*,
    },
};

/** `GET /orders/<id>` */
#[get("/<id>")]
pub fn get(id: OrderId, resolver: State<Resolver>) -> Result<Json<OrderWithProducts>, Error> {
    let query = resolver.get_order_with_products_query();

    match query.get_order_with_products(GetOrderWithProducts { id })? {
        Some(order) => Ok(Json(order)),
        None => Err(Error::NotFound(error::msg("order not found"))),
    }
}

#[derive(Deserialize)]
pub struct Create {
    pub customer: CustomerId,
}

/** `PUT /orders` */
#[put("/", format = "application/json", data = "<data>")]
pub fn create(data: Json<Create>, resolver: State<Resolver>) -> Result<Json<OrderId>, Error> {
    let id = resolver.order_id();
    let mut command = resolver.create_order_command();

    let id = id.get()?;

    command.create_order(CreateOrder {
        id,
        customer_id: data.customer,
    })?;

    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct ProductQuantity {
    quantity: u32,
}

/** `POST /orders/<id>/products/<product_id>` */
#[post(
    "/<id>/products/<product_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn add_or_update_product(
    id: OrderId,
    product_id: ProductId,
    data: Json<ProductQuantity>,
    resolver: State<Resolver>,
) -> Result<Json<LineItemId>, Error> {
    let mut command = resolver.add_or_update_product_command();

    let line_item_id = command.add_or_update_product(AddOrUpdateProduct {
        id,
        product_id,
        quantity: data.0.quantity,
    })?;

    Ok(Json(line_item_id))
}
