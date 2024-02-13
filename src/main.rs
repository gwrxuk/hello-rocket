#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;

use juniper::{FieldResult};

#[derive(GraphQLEnum)]
enum Product {
    MP3,
    MP4,
    Image,
}

#[derive(GraphQLObject)]
#[graphql(description="Shopping cart")]
struct ShoppingCart {
    id: String,
    user_id: String,
    product_ids: Vec<Product>,
    updated_at: String,
}

#[get("/")]
fn index()->&'static str {
    "hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
