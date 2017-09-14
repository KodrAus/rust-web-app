use std::convert::TryFrom;

use rocket::State;
use rocket_contrib::Json;

use domain::Resolver;
use domain::id::IdProvider;
use domain::products::*;

#[derive(Serialize)]
pub struct Get {
    pub id: String,
    pub title: String,
    pub price: f32,
}

#[get("/<id>")]
fn get(id: String, resolver: State<Resolver>) -> Result<Json<Get>, QueryError> {
    let query = resolver.get_product_query();

    let id = ProductId::try_from(&id)?;

    let product = query.get_product(GetProduct { id: id })?.into_data();

    Ok(Json(Get {
        id: product.id.to_string(),
        title: product.title,
        price: product.price,
    }))
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
) -> Result<Json<ProductId>, CreateProductError> {
    let id_provider = resolver.products().product_id_provider();
    let mut command = resolver.create_product_command();

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
    let mut command = resolver.set_product_title_command();

    let id = ProductId::try_from(&id)?;

    command.set_product_title(SetProductTitle {
        id: id,
        title: title,
    })?;

    Ok(())
}
