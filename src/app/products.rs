/*! `/products` */

use rocket::State;
use rocket_contrib::Json;

use app::error::Error;

use domain::Resolver;
use domain::id::IdProvider;
use domain::products::*;

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

    let product = query.get_product(GetProduct { id: id })?.into_data();

    Ok(Json(Get {
        id: product.id,
        title: product.title,
        price: product.price,
    }))
}

#[derive(Deserialize)]
pub struct Create {
    pub title: String,
    pub price: f32,
}

/** `PUT /products` */
#[put("/", format = "application/json", data = "<data>")]
pub fn create(data: Json<Create>, resolver: State<Resolver>) -> Result<Json<ProductId>, Error> {
    let id_provider = resolver.product_id_provider();
    let mut command = resolver.create_product_command();

    let id = id_provider.id()?;

    command.create_product(CreateProduct {
        id: id,
        title: data.0.title,
        price: data.0.price,
    })?;

    Ok(Json(id))
}

/** `POST /products/<id>/title/<title>` */
#[post("/<id>/title/<title>")]
fn set_title(id: ProductId, title: String, resolver: State<Resolver>) -> Result<(), Error> {
    let mut command = resolver.set_product_title_command();

    command.set_product_title(SetProductTitle {
        id: id,
        title: title,
    })?;

    Ok(())
}
