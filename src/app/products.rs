use std::str::FromStr;

use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::*;
use domain::products::*;

#[get("/<id>")]
fn get(id: String, resolver: State<Resolver>) -> Result<Json<GetProductResult>, QueryError> {
    let query = resolver.products().get_product_query();

    let id = Id::from_str(&id)?;

    let product = query.get_product(GetProduct { id: ProductId(id) })?;

    Ok(Json(product))
}

#[put("/", format = "application/json", data = "<data>")]
fn create(
    data: Json<CreateProduct>,
    resolver: State<Resolver>,
) -> Result<(), SetProductTitleError> {
    let mut command = resolver.products().create_product_command();

    command.create_product(data.0)?;

    Ok(())
}

#[post("/<id>/title/<title>")]
fn set_title(
    id: String,
    title: String,
    resolver: State<Resolver>,
) -> Result<(), SetProductTitleError> {
    let mut command = resolver.products().set_product_title_command();

    let id = Id::from_str(&id)?;

    command.set_product_title(SetProductTitle {
        id: ProductId(id),
        title: title,
    })?;

    Ok(())
}
