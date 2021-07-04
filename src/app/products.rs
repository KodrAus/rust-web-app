/*! `/products` */

use rocket::State;
use rocket_contrib::json::Json;

use crate::{
    app::error::{
        self,
        Error,
    },
    domain::{
        infra::*,
        products::*,
    },
};

#[derive(Serialize)]
pub struct Get {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
}

/** `GET /products/<id>` */
#[get("/<id>")]
pub fn get(id: ProductId, resolver: State<Resolver>) -> Result<Json<Get>, Error> {
    let query = resolver.get_product_query();

    match query.get_product(GetProduct { id })? {
        Some(product) => {
            let product = product.into_data();

            Ok(Json(Get {
                id: product.id,
                title: product.title,
                price: product.price,
            }))
        }
        None => Err(Error::NotFound(error::msg("product not found"))),
    }
}

#[derive(Deserialize)]
pub struct Create {
    pub title: String,
    pub price: f32,
}

/** `PUT /products` */
#[put("/", format = "application/json", data = "<data>")]
pub fn create(data: Json<Create>, resolver: State<Resolver>) -> Result<Json<ProductId>, Error> {
    let id = resolver.product_id();
    let mut command = resolver.create_product_command();

    let id = id.get()?;

    command.create_product(CreateProduct {
        id,
        title: data.0.title,
        price: data.0.price,
    })?;

    Ok(Json(id))
}

/** `POST /products/<id>/title/<title>` */
#[post("/<id>/title/<title>")]
pub fn set_title(id: ProductId, title: String, resolver: State<Resolver>) -> Result<(), Error> {
    let mut command = resolver.set_product_title_command();

    command.set_product_title(SetProductTitle { id, title })?;

    Ok(())
}
