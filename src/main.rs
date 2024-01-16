#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fire")]
fn fire() -> &'static str {
    "🚀"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, fire])
}
