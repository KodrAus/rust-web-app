/*! `/orders` */

use rocket::{
    response::status::Created,
    serde::json::Json,
};

use crate::{
    api::infra::*,
    domain::{
        customers::*,
        infra::*,
        orders::*,
        products::*,
    },
};

/** `GET /orders/<id>` */
#[rocket::get("/<id>")]
pub async fn get(id: OrderId, app: AppRequest<'_>) -> Result<Json<OrderWithProducts>, Error> {
    app.transaction(|app| async move {
        let query = app.get_order_with_products_query();

        match query.execute(GetOrderWithProducts { id }).await? {
            Some(order) => Ok(Json(order)),
            None => Err(Error::NotFound(error::msg("order not found"))),
        }
    })
    .await
}

/** `GET /orders/<id>/line-items/<line_item_id>` */
#[rocket::get("/<id>/line-items/<line_item_id>")]
pub async fn get_line_item(
    id: OrderId,
    line_item_id: LineItemId,
    app: AppRequest<'_>,
) -> Result<Json<LineItemWithProduct>, Error> {
    app.transaction(|app| async move {
        let query = app.get_line_item_with_product_query();

        match query
            .execute(GetLineItemWithProduct { id, line_item_id })
            .await?
        {
            Some(order) => Ok(Json(order)),
            None => Err(Error::NotFound(error::msg("order not found"))),
        }
    })
    .await
}

#[derive(Deserialize)]
pub struct Create {
    pub customer: CustomerId,
}

/** `PUT /orders` */
#[rocket::put("/", format = "application/json", data = "<data>")]
pub async fn create(
    data: Json<Create>,
    app: AppRequest<'_>,
) -> Result<Created<Json<OrderId>>, Error> {
    app.transaction(|app| async move {
        let id = app.order_id();
        let command = app.create_order_command();

        let id = id.get()?;

        command
            .execute(CreateOrder {
                id,
                customer_id: data.customer,
            })
            .await?;

        let location = format!("/orders/{}", id);

        Ok(Created::new(location).body(Json(id)))
    })
    .await
}

#[derive(Deserialize)]
pub struct ProductQuantity {
    quantity: u32,
}

/** `POST /orders/<id>/products/<product_id>` */
#[rocket::post(
    "/<id>/products/<product_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn add_or_update_product(
    id: OrderId,
    product_id: ProductId,
    data: Json<ProductQuantity>,
    app: AppRequest<'_>,
) -> Result<Json<LineItemId>, Error> {
    app.transaction(|app| async move {
        let command = app.add_or_update_product_command();

        let line_item_id = command
            .execute(AddOrUpdateProduct {
                id,
                product_id,
                quantity: data.0.quantity,
            })
            .await?;

        Ok(Json(line_item_id))
    })
    .await
}
