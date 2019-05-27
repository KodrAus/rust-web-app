/*! `/orders` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::Error,
    domain::{
        customers::*,
        id::IdProvider,
        orders::*,
        products::*,
        Resolver,
    },
};

/** `GET /orders/<id>` */
#[get("/<id>")]
pub fn get(id: OrderId, resolver: State<Resolver>) -> Result<Json<OrderWithProducts>, Error> {
    let query = resolver.get_order_with_products_query();

    let order = query.get_order_with_products(GetOrderWithProducts { id: id })?;

    Ok(Json(order))
}

#[derive(Deserialize)]
pub struct Create {
    pub customer: CustomerId,
}

/** `PUT /orders` */
#[put("/", format = "application/json", data = "<data>")]
pub fn create(data: Json<Create>, resolver: State<Resolver>) -> Result<Json<OrderId>, Error> {
    let id_provider = resolver.order_id_provider();
    let mut command = resolver.create_order_command();

    let id = id_provider.id()?;

    command.create_order(CreateOrder {
        id: id,
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
        id: id,
        product_id: product_id,
        quantity: data.0.quantity,
    })?;

    Ok(Json(line_item_id))
}
