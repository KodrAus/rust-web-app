/*! `/customers` */

use rocket::{
    response::status::Created,
    State,
};
use rocket_contrib::json::Json;

use crate::{
    api::error::{
        self,
        Error,
    },
    domain::{
        customers::*,
        infra::*,
    },
};

/** `GET /customers/<id>` */
#[get("/<id>")]
pub fn get(id: CustomerId, app: State<Resolver>) -> Result<Json<CustomerWithOrders>, Error> {
    let query = app.get_customer_with_orders_query();

    match query.get_customer_with_orders(GetCustomerWithOrders { id })? {
        Some(customer) => Ok(Json(customer)),
        None => Err(Error::NotFound(error::msg("customer not found"))),
    }
}

/** `PUT /customers` */
#[put("/", format = "application/json")]
pub fn create(app: State<Resolver>) -> Result<Created<Json<CustomerId>>, Error> {
    let id = app.customer_id();

    let mut command = app.create_customer_command();

    let id = id.get()?;

    command.create_customer(CreateCustomer { id })?;

    let location = format!("/customers/{}", id);

    Ok(Created(location, Some(Json(id))))
}
