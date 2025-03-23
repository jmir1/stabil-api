use axum::{
    extract::{Query, State},
    http::StatusCode, Json,
};

use crate::{
    scraping::{
        models::{ApiResponse, ApiResult, Reservation, SessionTokenQuery},
        utils::is_logged_in,
    }, ReservationData
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
    let url = format!(
        "https://katalogplus.sub.uni-hamburg.de/vufind/Record/{ppn}/Hold?doc_id={doc_id}&item_id={item_id}&hashKey={hash_key}&type=hold",
        ppn = data.ppn,
        doc_id = data.doc_id,
        item_id = data.item_id,
        hash_key = data.hash_key
    );
    let response = state.no_redirect_client
        .get(url)
        .header("cookie", format!("VUFIND_SESSION={session_token}"))
        .send()
        .await
        .unwrap();
    let status = response.status();
    println!("status: {}", status);
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
        let text = response.text().await.unwrap();
        print!("text: {}", text);
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
        //let message = document.select_all("selector").first().unwrap_or();
        let result = ApiResult {
            success: false,
            data: false,
            msg: "Reservation unsuccessful.".to_string(),
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
