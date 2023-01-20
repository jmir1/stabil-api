#[macro_use]
extern crate rocket;

mod routes;
mod scraping;

#[launch]
fn rocket() -> _ {
    let client = reqwest::Client::new();

    rocket::build().manage(client).mount(
        "/",
        routes![
            routes::get_index::route,
            routes::post_session_token::route,
            routes::get_loans::route,
            routes::get_loans::default_route,
        ],
    )
}
