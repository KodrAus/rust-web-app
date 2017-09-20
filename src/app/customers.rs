use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::IdProvider;
use domain::customers::*;

#[put("/", format = "application/json")]
fn create(resolver: State<Resolver>) -> Result<Json<CustomerId>, CreateCustomerError> {
    let id_provider = resolver.customers().customer_id_provider();
    let mut command = resolver.create_customer_command();

    let id = id_provider.id()?;

    command.create_customer(CreateCustomer { id: id })?;

    Ok(Json(id))
}
