use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use std::ops::Add;
use utoipa::ToSchema;

use crate::scraping::{
    models::{ApiResponse, ApiResult, Session},
    utils::Select,
};

#[utoipa::path(
    post,
    path = "/session_token",
    request_body = LoginData,
    responses(
        (status = 200, description = "Session token", body = ApiResult<Session>),
        (status = 401, description = "Unauthorized", body = String),
    )
)]
#[worker::send]
pub async fn route(
    State(state): State<crate::State>,
    login_data: Json<LoginData>,
) -> ApiResponse<Session> {
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
                result: ApiResult {
                    success: false,
                    data: Session {
                        session_token: String::new(),
                        expiry: -1,
                    },
                    msg: "Failed to connect to the library server.".to_string(),
                },
            };
        }
        Ok(response) => (
            response.headers().to_owned(),
            response.text().await.unwrap_or_default(),
        ),
    };

    let session_token = match get_token_from_headers(login_response_headers) {
        Some(token) => token,
        None => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: Session {
                        session_token: String::new(),
                        expiry: -1,
                    },
                    msg: "Failed to get session token.".to_string(),
                },
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
        None => String::new(),
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
    let (session, status, msg) = if success {
        (
            Session {
                session_token,
                expiry,
            },
            StatusCode::OK.as_u16(),
            String::new(),
        )
    } else {
        (
            Session {
                session_token: String::new(),
                expiry: -1,
            },
            StatusCode::UNAUTHORIZED.as_u16(),
            "Login details seem to be incorrect.".to_string(),
        )
    };
    let result = ApiResult {
        success,
        data: session,
        msg,
    };
    ApiResponse { status, result }
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
        None => return None,
    };
    let token_end = session_cookie.find(';').unwrap_or(session_cookie.len());
    Some(session_cookie[token_start..token_end].to_string())
}
