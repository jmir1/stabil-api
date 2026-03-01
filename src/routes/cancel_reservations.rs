use serde::Deserialize;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use utoipa::ToSchema;

use crate::scraping::{
    models::{ApiResponse, ApiResponseBody, SessionTokenQuery},
    utils::{is_logged_in, Select},
};

#[derive(Deserialize, ToSchema)]
pub struct CancelReservationData {
    pub to_cancel: Vec<String>,
}

#[utoipa::path(
  operation_id = "post_cancel_reservations",
  post,
  path = "/cancel_reservations",
  request_body = CancelReservationData,
  responses(
    (status = 200, description = "Reservations canceled successfully", body = String),
    (status = 400, description = "Bad request", body = String),
    (status = 401, description = "Unauthorized", body = String),
  ),
  security(("session_token" = [])),
)]
#[worker::send]
pub async fn post(
    State(state): State<crate::State>,
    query: Query<SessionTokenQuery>,
    Json(data): Json<CancelReservationData>,
) -> ApiResponse {
    let session_token = match &query.session_token {
        Some(token) => token,
        None => return default_route(),
    };

    let mut form_body =
        reqwest::multipart::Form::new().text("cancelSelected", "Bestellung stornieren");

    for id in data.to_cancel {
        form_body = form_body.text("selectedIDS[]", id);
    }

    let response = match state
        .client
        .post("https://katalogplus.sub.uni-hamburg.de/vufind/Holds/List")
        .multipart(form_body)
        .header(
            "cookie",
            format!("VUFIND_SESSION={session_token}; ui=standard"),
        )
        .send()
        .await
    {
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                body: ApiResponseBody::Text("Failed to connect to the library server.".to_string()),
            };
        }
        Ok(response) => response,
    };

    let response_text = response.text().await.unwrap_or_default();
    let document = scraper::Html::parse_document(&response_text);

    if !is_logged_in(&document) {
        return ApiResponse {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            body: ApiResponseBody::Text("session_token is invalid.".to_string()),
        };
    }

    let alert_message = document
        .select_first("div[role=alert]")
        .and_then(|element| element.text().next())
        .unwrap_or("Failed to cancel reservations.")
        .to_string();

    if alert_message.contains("erfolgreich storniert") {
        ApiResponse {
            status: StatusCode::OK.as_u16(),
            body: ApiResponseBody::Text("Reservations canceled successfully.".to_string()),
        }
    } else {
        ApiResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            body: ApiResponseBody::Text(alert_message),
        }
    }
}

pub fn default_route() -> ApiResponse {
    let msg = "This route needs a session_token query parameter.".to_string();
    ApiResponse {
        status: StatusCode::BAD_REQUEST.as_u16(),
        body: ApiResponseBody::Text(msg),
    }
}
