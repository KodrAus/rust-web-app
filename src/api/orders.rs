/*! `/orders` */

use rocket::{
    response::status::Created,
    serde::json::Json,
    State,
};

use crate::{
    api::error::{
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
pub async fn get(id: OrderId, app: &State<App>) -> Result<Json<OrderWithProducts>, Error> {
    app.transaction(|app| async move {
        let query = app.get_order_with_products_query();

        match query.execute(GetOrderWithProducts { id }).await? {
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
#[put("/", format = "application/json", data = "<data>")]
pub async fn create(data: Json<Create>, app: &State<App>) -> Result<Created<Json<OrderId>>, Error> {
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
#[post(
    "/<id>/products/<product_id>",
    format = "application/json",
    data = "<data>"
)]
pub async fn add_or_update_product(
    id: OrderId,
    product_id: ProductId,
    data: Json<ProductQuantity>,
    app: &State<App>,
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
