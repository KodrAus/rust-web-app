use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::products::*;

#[get("/<id>")]
fn get(id: i32, resolver: State<Resolver>) -> Result<Json<GetProductResult>, QueryError> {
    let query = resolver.get_product_query();

    let product = query.get_product(GetProduct { id: id })?;

    Ok(Json(product))
}

#[post("/<id>/<title>")]
fn post(id: i32, title: String, resolver: State<Resolver>) -> Result<(), CommandError> {
    let mut command = resolver.set_product_command();

    command.set_product(SetProduct { id: id, title: title })?;

    Ok(())
}
