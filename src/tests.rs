#[cfg(test)]
mod tests {
    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_login() {
        use axum::{extract::State, http::StatusCode, Json};

        use crate::{routes::post_session_token::LoginData};

        let username = std::env::var("STABIL_API_TEST_USERNAME")
            .expect("STABIL_API_TEST_USERNAME env var not provided!");
        let password = std::env::var("STABIL_API_TEST_PASSWORD")
            .expect("STABIL_API_TEST_PASSWORD env var not provided!");

        let response = crate::routes::post_session_token::route(
            State(
                crate::State {
                    client: reqwest::Client::new(),
                    no_redirect_client: reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap()
                }
            ),
            Json(LoginData { username, password }),
        )
        .await;

        assert_eq!(response.status, StatusCode::OK.as_u16());
        assert!(response.result.success);
        assert!(response.result.data.expiry > 0);
    }

    #[cfg(not(feature = "server"))]
    #[test]
    fn test_login() {
        println!("The test requires the server feature / the tokio dependency.");
    }
}
