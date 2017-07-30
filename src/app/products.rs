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

#[put("/", format = "application/json", data = "<data>")]
fn create(data: Json<CreateProduct>, resolver: State<Resolver>) -> Result<(), SetProductTitleError> {
    let mut command = resolver.create_product_command();

    command.create_product(data.0)?;

    Ok(())
}

#[post("/<id>/title/<title>")]
fn set_title(id: i32, title: String, resolver: State<Resolver>) -> Result<(), SetProductTitleError> {
    let mut command = resolver.set_product_title_command();

    command.set_product_title(SetProductTitle { id: id, title: title })?;

    Ok(())
}
