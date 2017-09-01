use std::convert::TryFrom;

use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::IdProvider;
use domain::products::*;

#[get("/<id>")]
fn get(id: String, resolver: State<Resolver>) -> Result<Json<GetProductResult>, QueryError> {
    let query = resolver.products().get_product_query();

    let id = ProductId::try_from(&id)?;

    let product = query.get_product(GetProduct { id: id })?;

    Ok(Json(product))
}

#[derive(Deserialize)]
pub struct Create {
    pub title: String,
    pub price: f32,
}

#[put("/", format = "application/json", data = "<data>")]
fn create(
    data: Json<Create>,
    resolver: State<Resolver>,
) -> Result<Json<ProductId>, SetProductTitleError> {
    let id_provider = resolver.products().product_id_provider();
    let mut command = resolver.products().create_product_command();

    let id = id_provider.id()?;

    command.create_product(CreateProduct {
        id: id,
        title: data.0.title,
        price: data.0.price,
    })?;

    Ok(Json(id))
}

#[post("/<id>/title/<title>")]
fn set_title(
    id: String,
    title: String,
    resolver: State<Resolver>,
) -> Result<(), SetProductTitleError> {
    let mut command = resolver.products().set_product_title_command();

    let id = ProductId::try_from(&id)?;

    command.set_product_title(SetProductTitle {
        id: id,
        title: title,
    })?;

    Ok(())
}
