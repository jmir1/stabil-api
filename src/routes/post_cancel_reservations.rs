use serde::Deserialize;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use utoipa::ToSchema;

use crate::scraping::{
    models::{ApiResponse, ApiResult, SessionTokenQuery},
    utils::{is_logged_in, Select},
};

#[derive(Deserialize, ToSchema)]
pub struct CancelReservationData {
    pub to_cancel: Vec<String>,
}

#[utoipa::path(
  post,
  path = "/cancel_reservations",
  request_body = CancelReservationData,
  responses(
    (status = 200, description = "Reservations canceled successfully", body = ApiResult<bool>),
    (status = 400, description = "Bad request", body = String),
    (status = 401, description = "Unauthorized", body = String),
  ),
  security(("session_token" = [])),
)]
#[worker::send]
pub async fn route(
    State(state): State<crate::State>,
    query: Query<SessionTokenQuery>,
    Json(data): Json<CancelReservationData>,
) -> ApiResponse<bool> {
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
                result: ApiResult {
                    success: false,
                    data: false,
                    msg: "Failed to connect to the library server.".to_string(),
                },
            };
        }
        Ok(response) => response,
    };

    let response_text = response.text().await.unwrap_or_default();
    let document = scraper::Html::parse_document(&response_text);

    if !is_logged_in(&document) {
        let result = ApiResult {
            success: false,
            data: false,
            msg: "session_token is invalid.".to_string(),
        };
        return ApiResponse {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            result,
        };
    }

    let alert_message = document
        .select_first("div[role=alert]")
        .and_then(|element| element.text().next())
        .unwrap_or("Failed to cancel reservations.")
        .to_string();

    if alert_message.contains("erfolgreich storniert") {
        let result = ApiResult {
            success: true,
            data: true,
            msg: "Reservations canceled successfully.".to_string(),
        };
        ApiResponse {
            status: StatusCode::OK.as_u16(),
            result,
        }
    } else {
        let result = ApiResult {
            success: false,
            data: false,
            msg: alert_message,
        };
        ApiResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            result,
        }
    }
}

pub fn default_route() -> ApiResponse<bool> {
    let msg = "This route needs a session_token query parameter.".to_string();
    let result = ApiResult {
        success: false,
        data: false,
        msg,
    };
    ApiResponse {
        status: StatusCode::BAD_REQUEST.as_u16(),
        result,
    }
}
