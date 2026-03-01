use axum::{
    routing::{get, post},
    Router,
};
use scraping::models::*;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[cfg(test)]
mod tests;

mod routes;
mod scraping;

#[derive(OpenApi)]
#[openapi(
        paths(
            routes::cancel_reservations::post,
            routes::checked_out::get,
            routes::index::get,
            routes::make_reservation::post,
            routes::ppn_from_bar::get,
            routes::reservations::get,
            routes::search::get,
            routes::session_token::post,
        ),
        components(
            schemas(
                Session,
                CheckedOut,
                Volume,
                Medium,
                Location,
                Status,
                Reservation,
                Library,
                routes::session_token::LoginData,
            ),
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "stabil-api", description = "API for interacting with the University of Hamburg's library system")
        )
    )]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_token",
                SecurityScheme::ApiKey(ApiKey::Query(ApiKeyValue::new("session_token"))),
            )
        }
    }
}
#[derive(Clone)]
pub struct State {
    client: reqwest::Client,
    no_redirect_client: reqwest::Client,
}

pub fn router() -> Router {
    let client = reqwest::Client::new();
    let no_redirect_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build no-redirect client");
    let state = State {
        client,
        no_redirect_client,
    };
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(routes::index::get))
        .route("/session_token", post(routes::session_token::post))
        .route("/checked_out", get(routes::checked_out::get))
        .route("/ppn_from_bar", get(routes::ppn_from_bar::get))
        .route("/reservations", get(routes::reservations::get))
        .route("/search", get(routes::search::get))
        .route(
            "/cancel_reservations",
            post(routes::cancel_reservations::post),
        )
        .route("/make_reservation", post(routes::make_reservation::post))
        .with_state(state)
}
/*
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
*/
