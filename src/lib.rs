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
            routes::get_index::route,
            routes::post_session_token::route,
            routes::get_checked_out::route,
            routes::get_reservations::route,
            routes::post_make_reservation::route,
            routes::post_cancel_reservations::route,
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
                ApiResult<bool>,
                ApiResult<Session>,
                ApiResult<Vec<CheckedOut>>,
                ApiResult<Vec<Reservation>>,
                ApiResult<Vec<Medium>>,
                routes::post_session_token::LoginData,
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
        .route("/", get(routes::get_index::route))
        .route("/session_token", post(routes::post_session_token::route))
        .route("/checked_out", get(routes::get_checked_out::route))
        .route("/reservations", get(routes::get_reservations::route))
        .route("/search", get(routes::get_search::route))
        .route(
            "/cancel_reservations",
            post(routes::post_cancel_reservations::route),
        )
        .route(
            "/make_reservation",
            post(routes::post_make_reservation::route),
        )
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
