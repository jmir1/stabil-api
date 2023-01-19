use crate::scraping::{utils::Select, models::ApiResult};

#[post("/session_token", data = "<login_data>")]
pub async fn route(
    login_data: rocket::form::Form<LoginData>,
    client: &rocket::State<reqwest::Client>,
) -> String {
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

    let request = client
    .post("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/Home?layout=lightbox&lbreferer=https%3A%2F%2Fkatalogplus.sub.uni-hamburg.de%2Fvufind%2FMyResearch%2FUserLogin")
    .multipart(form_body)
        .header("cookie", format!("VUFIND_SESSION={session_token}"));

    let status = request.send().await.unwrap().status().as_u16();
    let success = status == 205;
    let result = ApiResult { success, data: session_token, msg: String::new() };
    serde_json::to_string(&result).unwrap()
}

#[derive(FromForm)]
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
