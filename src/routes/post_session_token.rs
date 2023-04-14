use std::ops::Add;

use crate::scraping::{
    models::{ApiResponse, ApiResult, Session},
    utils::Select,
};

#[rocket_okapi::openapi(tag = "Receive a session_token for accessing the user's logged in area.")]
#[post("/session_token", data = "<login_data>")]
pub async fn route(
    login_data: rocket::serde::json::Json<LoginData>,
    client: &rocket::State<reqwest::Client>,
) -> ApiResponse<Session> {
    let username = login_data.username.to_owned();
    let password = login_data.password.to_owned();

    let login_response = client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/UserLogin")
        .send()
        .await
        .unwrap();
    let login_response_headers = login_response.headers().to_owned();
    let session_token = get_token_from_headers(login_response_headers);

    let login_response_text = login_response.text().await.unwrap();
    let csrf = scraper::Html::parse_document(&login_response_text)
        .select_first("input[type=hidden][name=csrf]")
        .value()
        .attr("value")
        .unwrap()
        .to_string();

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
    let request = client
    .post("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/Home?layout=lightbox&lbreferer=https%3A%2F%2Fkatalogplus.sub.uni-hamburg.de%2Fvufind%2FMyResearch%2FUserLogin")
    .multipart(form_body)
    .header("cookie", format!("VUFIND_SESSION={session_token}"));

    let status = request.send().await.unwrap().status().as_u16();
    let success = status == 205;
    let (session, status, msg) = if success {
        (
            Session {
                session_token,
                expiry,
            },
            rocket::http::Status::Ok.code,
            String::new(),
        )
    } else {
        (
            Session {
                session_token: String::new(),
                expiry: -1,
            },
            rocket::http::Status::Unauthorized.code,
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

#[derive(serde::Deserialize, serde::Serialize, rocket_okapi::JsonSchema)]
pub struct LoginData {
    username: String,
    password: String,
}

fn get_token_from_headers(headers: reqwest::header::HeaderMap) -> String {
    let session_cookie = headers
        .get_all("set-cookie")
        .iter()
        .filter(|cookie| cookie.to_str().unwrap().contains("VUFIND_SESSION="))
        .map(|cookie| cookie.to_str().unwrap())
        .next()
        .unwrap();
    let token_start = session_cookie.find("VUFIND_SESSION=").unwrap() + 15; // + "VUFIND_SESSION=".len();
    let token_end = session_cookie.find(';').unwrap_or(session_cookie.len());
    session_cookie[token_start..token_end].to_string()
}

#[rocket_okapi::openapi(skip)]
#[post("/session_token", rank = 0)]
pub fn default_route() -> ApiResponse<Session> {
    let msg = "You need to pass the username and password in the form body.".to_string();
    let result = ApiResult {
        success: false,
        data: Session {
            session_token: String::new(),
            expiry: -1,
        },
        msg,
    };
    ApiResponse {
        status: rocket::http::Status::BadRequest.code,
        result,
    }
}
