#[cfg(test)]
mod tests {
    use crate::scraping::models::ApiResponseBody;

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_login() {
        use axum::{extract::State, http::StatusCode, Json};

        use crate::routes::session_token::LoginData;

        let username = std::env::var("STABIL_API_TEST_USERNAME")
            .expect("STABIL_API_TEST_USERNAME env var not provided!");
        let password = std::env::var("STABIL_API_TEST_PASSWORD")
            .expect("STABIL_API_TEST_PASSWORD env var not provided!");

        let response = crate::routes::session_token::post(
            State(crate::State {
                client: reqwest::Client::new(),
                no_redirect_client: reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::none())
                    .build()
                    .expect("Failed to build no-redirect client"),
            }),
            Json(LoginData { username, password }),
        )
        .await;

        assert_eq!(response.status, StatusCode::OK.as_u16());
        match response.body {
            ApiResponseBody::Session(session) => {
                assert!(session.session_token.len() > 0);
                assert!(session.expiry > 0);
            }
            _ => panic!("Expected Session in response body"),
        }
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn test_get_ppn_from_bar() {
        use axum::{extract::Query, http::StatusCode};

        use crate::routes::ppn_from_bar::get;
        let response = get(Query(crate::scraping::models::BarcodeQuery {
            barcode: Some(109956811),
        }))
        .await;

        assert_eq!(response.status, StatusCode::OK.as_u16());
        match response.body {
            ApiResponseBody::Number(ppn) => assert_eq!(ppn, 1947771086),
            _ => panic!("Expected a number in response data"),
        }
    }

    #[cfg(not(feature = "server"))]
    #[test]
    fn test_login() {
        println!("The test requires the server feature / the tokio dependency.");
    }
}
