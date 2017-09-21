use std::convert::TryFrom;

use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::IdProvider;
use domain::customers::*;

pub type Error = String;

#[get("/<id>")]
fn get(id: String, resolver: State<Resolver>) -> Result<Json<CustomerWithOrders>, Error> {
    let query = resolver.get_customer_with_orders_query();

    let id = CustomerId::try_from(&id)?;

    let order = query.get_customer_with_orders(GetCustomerWithOrders { id: id })?;

    Ok(Json(order))
}

#[put("/", format = "application/json")]
fn create(resolver: State<Resolver>) -> Result<Json<CustomerId>, Error> {
    let id_provider = resolver.customer_id_provider();
    let mut command = resolver.create_customer_command();

    let id = id_provider.id()?;

    command.create_customer(CreateCustomer { id: id })?;

    Ok(Json(id))
}
