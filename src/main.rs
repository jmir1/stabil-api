#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

#[cfg(test)]
mod tests;

mod routes;
mod scraping;

#[launch]
fn rocket() -> _ {
    let client = reqwest::Client::new();

    rocket::build()
        .manage(client)
        .mount(
            "/",
            openapi_get_routes![
                routes::get_index::route,
                routes::post_session_token::route,
                routes::post_session_token::default_route,
                routes::get_checked_out::route,
                routes::get_checked_out::default_route,
                routes::get_reservations::route,
                routes::get_reservations::default_route,
            ],
        )
        .mount(
            "/swagger-ui/",
            rocket_okapi::swagger_ui::make_swagger_ui(&rocket_okapi::swagger_ui::SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
