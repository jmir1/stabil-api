use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use std::ops::Add;
use utoipa::ToSchema;

use crate::scraping::{
    models::{ApiResponse, ApiResponseBody, Session},
    utils::Select,
};

#[utoipa::path(
    operation_id = "post_session_token",
    post,
    path = "/session_token",
    request_body = LoginData,
    responses(
        (status = 200, description = "Session token", body = Session),
        (status = 401, description = "Unauthorized", body = String),
    )
)]
#[worker::send]
pub async fn post(State(state): State<crate::State>, login_data: Json<LoginData>) -> ApiResponse {
    let username = login_data.username.to_owned();
    let password = login_data.password.to_owned();

    let login_response = state
        .client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/UserLogin")
        .send()
        .await;
    let (login_response_headers, login_response_text) = match login_response {
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                body: ApiResponseBody::Text("Failed to connect to the library server.".to_string()),
            };
        }
        Ok(response) => (
            response.headers().to_owned(),
            response.text().await.unwrap_or_default(),
        ),
    };

    let session_token = match get_token_from_headers(login_response_headers) {
        Some(token) => token,
        _ => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                body: ApiResponseBody::Text("Failed to get session token.".to_string()),
            };
        }
    };

    let document = scraper::Html::parse_document(&login_response_text);
    let csrf = match document.select_first("input[type=hidden][name=csrf]") {
        Some(element) => element
            .value()
            .attr("value")
            .unwrap_or_default()
            .to_string(),
        _ => String::new(),
    };

    let form_body = reqwest::multipart::Form::new()
        .text("csrf", csrf.to_owned())
        .text("username", username)
        .text("password", password)
        .text("auth_method", "ILS")
        .text("csrf", csrf)
        .text("processLogin", "Anmelden");

    let expiry = chrono::Utc::now()
        .add(chrono::Duration::hours(1))
        .timestamp(); // The session expires 1 hour from now.
    let request = state.no_redirect_client
    .post("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/Home?layout=lightbox&lbreferer=https%3A%2F%2Fkatalogplus.sub.uni-hamburg.de%2Fvufind%2FMyResearch%2FUserLogin")
    .multipart(form_body)
    .header("cookie", format!("VUFIND_SESSION={session_token}"));

    let success = match request.send().await {
        Err(_) => false,
        Ok(res) => res.status() == StatusCode::RESET_CONTENT,
    };
    if success {
        let session = Session {
            session_token,
            expiry,
        };
        ApiResponse {
            status: StatusCode::OK.as_u16(),
            body: ApiResponseBody::Session(session),
        }
    } else {
        ApiResponse {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            body: ApiResponseBody::Text("Login details seem to be incorrect.".to_string()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

fn get_token_from_headers(headers: reqwest::header::HeaderMap) -> Option<String> {
    let session_cookie = headers
        .get_all("set-cookie")
        .iter()
        .filter(|cookie| {
            cookie
                .to_str()
                .unwrap_or_default()
                .contains("VUFIND_SESSION=")
        })
        .map(|cookie| cookie.to_str().unwrap_or_default())
        .next()
        .unwrap_or_default();
    let token_start = match session_cookie.find("VUFIND_SESSION=") {
        Some(start) => start + 15, // 15 == "VUFIND_SESSION=".len()
        _ => return None,
    };
    let token_end = session_cookie.find(';').unwrap_or(session_cookie.len());
    Some(session_cookie[token_start..token_end].to_string())
}
