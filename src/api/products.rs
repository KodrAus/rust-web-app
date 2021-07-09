/*! `/products` */

use rocket::{
    response::status::Created,
    serde::json::Json,
    State,
};

use crate::{
    api::error::{
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
    pub price: Currency,
}

/** `GET /products/<id>` */
#[get("/<id>")]
pub async fn get(id: ProductId, app: &State<App>) -> Result<Json<Get>, Error> {
    app.transaction(|app| {
        let query = app.get_product_query();

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
    })
}

#[derive(Deserialize)]
pub struct Create {
    pub title: String,
    pub price: Currency,
}

/** `PUT /products` */
#[put("/", format = "application/json", data = "<data>")]
pub async fn create(
    data: Json<Create>,
    app: &State<App>,
) -> Result<Created<Json<ProductId>>, Error> {
    app.transaction(|app| {
        let id = app.product_id();
        let mut command = app.create_product_command();

        let id = id.get()?;

        command.create_product(CreateProduct {
            id,
            title: data.0.title,
            price: data.0.price,
        })?;

        let location = format!("/products/{}", id);

        Ok(Created::new(location).body(Json(id)))
    })
}

/** `POST /products/<id>/title/<title>` */
#[post("/<id>/title/<title>")]
pub async fn set_title(id: ProductId, title: String, app: &State<App>) -> Result<(), Error> {
    app.transaction(|app| {
        let mut command = app.set_product_title_command();

        command.set_product_title(SetProductTitle { id, title })?;

        Ok(())
    })
}