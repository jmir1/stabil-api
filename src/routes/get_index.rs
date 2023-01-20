#[rocket_okapi::openapi(skip)]
#[get("/")]
pub fn route() -> &'static str {
    "Hello World!"
}
