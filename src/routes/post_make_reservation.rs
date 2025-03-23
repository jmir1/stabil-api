use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    scraping::{
        models::{ApiResponse, ApiResult, Reservation, SessionTokenQuery},
        utils::{is_logged_in, Select},
    },
    ReservationData,
};

#[utoipa::path(
    get,
    path = "/make_reservation",
    responses(
        (status = 200, description = "Reserved items", body = ApiResult<Vec<Reservation>>),
        (status = 400, description = "Bad request", body = String),
        (status = 401, description = "Unauthorized", body = String),
    ),
    security(("session_token" = [])),
)]
#[worker::send]
pub async fn route(
    State(state): State<crate::State>,
    query: Query<SessionTokenQuery>,
    data: Json<ReservationData>,
) -> ApiResponse<bool> {
    let session_token = match &query.session_token {
        Some(token) => token,
        None => return default_route(),
    };

    let mut url = reqwest::Url::parse(&format!(
        "https://katalogplus.sub.uni-hamburg.de/vufind/Record/{}/Hold",
        data.ppn
    ))
    .expect("Invalid base URL");
    url.query_pairs_mut()
        .append_pair("doc_id", &data.doc_id)
        .append_pair("item_id", &data.item_id)
        .append_pair("hashKey", &data.hash_key)
        .append_pair("type", "hold");

    let response = match state
        .no_redirect_client
        .get(url)
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
    let status = response.status();
    if status == StatusCode::FOUND {
        let result = ApiResult {
            success: true,
            data: true,
            msg: "Reservation successful.".to_string(),
        };
        return ApiResponse {
            status: StatusCode::OK.as_u16(),
            result,
        };
    } else {
        let text = response.text().await.unwrap_or_default();
        let document = scraper::Html::parse_document(&text);
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
        let message_element = document.select_first("div[role=alert]");
        let message: &str = match message_element {
            Some(element) => element.text().next().unwrap_or("Reservation unsuccessful."),
            None => "Reservation unsuccessful.",
        };
        let result = ApiResult {
            success: false,
            data: false,
            msg: message.to_string(),
        };
        return ApiResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            result,
        };
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
