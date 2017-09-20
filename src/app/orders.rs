use std::convert::TryFrom;

use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::IdProvider;
use domain::orders::*;
use domain::customers::*;
use domain::products::*;

#[derive(Deserialize)]
pub struct Create {
    pub customer: String,
}

#[put("/", format = "application/json", data = "<data>")]
fn create(data: Json<Create>, resolver: State<Resolver>) -> Result<Json<OrderId>, CreateOrderError> {
    let id_provider = resolver.orders().order_id_provider();
    let mut command = resolver.create_order_command();

    let id = id_provider.id()?;
    let customer_id = CustomerId::try_from(&data.customer)?;

    command.create_order(CreateOrder {
        id: id,
        customer_id: customer_id,
    })?;

    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct ProductQuantity {
    quantity: u32,
}

#[post("/<id>/products/<product_id>", format = "application/json", data = "<data>")]
fn add_or_update_product(id: String, product_id: String, data: Json<ProductQuantity>, resolver: State<Resolver>) -> Result<Json<LineItemId>, AddOrUpdateProductError> {
    let mut command = resolver.add_or_update_product_command();

    let id = OrderId::try_from(&id)?;
    let product_id = ProductId::try_from(&product_id)?;

    let line_item_id = command.add_or_update_product(AddOrUpdateProduct {
        id: id,
        product_id: product_id,
        quantity: data.0.quantity,
    })?;

    Ok(Json(line_item_id))
}
