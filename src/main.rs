#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

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
            "/rapidoc/",
            rocket_okapi::rapidoc::make_rapidoc(&rocket_okapi::rapidoc::RapiDocConfig {
                general: rocket_okapi::rapidoc::GeneralConfig {
                    spec_urls: vec![rocket_okapi::settings::UrlObject::new(
                        "General",
                        "../openapi.json",
                    )],
                    ..Default::default()
                },
                hide_show: rocket_okapi::rapidoc::HideShowConfig {
                    allow_authentication: false,
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
