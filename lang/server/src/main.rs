use rocket::{
    data::{Data, ToByteUnit},
    get,
    http::uri::Absolute,
    launch, main as rocket_main,
    response::content::Plain,
    routes,
    tokio::fs::{self, File},
};
use self::paste_id::PasteId;

mod paste_id;

const HOST: Absolute<'static> = uri!("http://localhost:8000");

const ID_LENGTH: usize = 3;

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[rocket_main]
async fn main() {
    rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await
        .unwrap();
}
