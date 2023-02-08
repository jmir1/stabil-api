#[cfg(test)]
mod tests {
    use crate::rocket;
    use crate::routes::post_session_token;
    use crate::scraping::models::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_login() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let username = std::env::var("STABIL_API_TEST_USERNAME")
            .expect("STABIL_API_TEST_USERNAME env var not provided!");
        let password = std::env::var("STABIL_API_TEST_PASSWORD")
            .expect("STABIL_API_TEST_PASSWORD env var not provided!");
        let response = client
            .post(uri!(post_session_token::route))
            .header(ContentType::Form)
            .body(format!("username={username}&password={password}"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let result: ApiResult<Session> = response.into_json().unwrap();
        assert!(result.success);
        assert!(result.data.expiry > 0);
    }
}
