use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn launch_rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
