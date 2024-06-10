/*! `/customers` */

use rocket::{
    response::status::Created,
    serde::json::Json,
};

use crate::{
    api::infra::*,
    domain::{
        customers::*,
        infra::*,
    },
};

/** `GET /customers/<id>` */
#[rocket::get("/<id>")]
pub async fn get(id: CustomerId, app: AppRequest<'_>) -> Result<Json<CustomerWithOrders>, Error> {
    app.transaction(|app| async move {
        let query = app.get_customer_with_orders_query();

        match query.execute(GetCustomerWithOrders { id }).await? {
            Some(customer) => Ok(Json(customer)),
            None => Err(Error::NotFound(error::msg("customer not found"))),
        }
    })
    .await
}

/** `PUT /customers` */
#[rocket::put("/", format = "application/json")]
pub async fn create(app: AppRequest<'_>) -> Result<Created<Json<CustomerId>>, Error> {
    app.transaction(|app| async move {
        let id = app.customer_id();

        let command = app.create_customer_command();

        let id = id.get()?;

        command.execute(CreateCustomer { id }).await?;

        let location = format!("/customers/{}", id);

        Ok(Created::new(location).body(Json(id)))
    })
    .await
}
