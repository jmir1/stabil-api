use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    scraping::{
        libraries::to_library,
        models::{
            ApiResponse,
            ApiResponseBody::{Reservations, Text},
            Location, Medium, Reservation, Volume,
        },
        utils::{get_medium_ppn_from_id, is_logged_in, Select},
    },
    SessionTokenQuery,
};

use super::checked_out::get_bar;

#[utoipa::path(
    operation_id = "get_reservations",
    get,
    path = "/reservations",
    responses(
        (status = 200, description = "Reserved items", body = Vec<Reservation>),
        (status = 400, description = "Bad request", body = String),
        (status = 401, description = "Unauthorized", body = String),
    ),
    security(("session_token" = [])),
)]
#[worker::send]
pub async fn get(
    State(state): State<crate::State>,
    query: Query<SessionTokenQuery>,
) -> ApiResponse {
    let session_token = match &query.session_token {
        Some(token) => token,
        None => return default_route(),
    };
    let response_text = match state
        .client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/Holds/List")
        .header("cookie", format!("VUFIND_SESSION={session_token}"))
        .send()
        .await
    {
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                body: Text("Failed to connect to the library server.".to_string()),
            };
        }
        Ok(response) => response.text().await.unwrap_or_default(),
    };
    let document = scraper::Html::parse_document(&response_text);
    if !is_logged_in(&document) {
        return ApiResponse {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            body: Text("session_token is invalid.".to_string()),
        };
    }

    let mut reservations: Vec<Reservation> = vec![];
    for reservation in document.select_all("tr.myresearch-result") {
        let ppn = get_medium_ppn_from_id(reservation.value().attr("id").unwrap_or_default());
        let title = match reservation.select_first("td[data-th=Titel] > span.title") {
            Some(element) => element.text().next().unwrap_or_default().to_string(),
            None => String::new(),
        };
        let signature = get_column_value(reservation, "Signatur");
        let due_date = get_column_value(reservation, "Rückgabedatum");
        let cancel_uri =
            match reservation.select_first("td > label > input[name=\"cancelSelectedIDS[]\"]") {
                Some(element) => element
                    .value()
                    .attr("value")
                    .unwrap_or_default()
                    .to_string(),
                None => String::new(),
            };

        let bar = get_bar(&cancel_uri);
        let library = to_library(
            match reservation.select_first("td[data-th=\"Standort (Printmedien)\"] > strong") {
                Some(element) => element.text().next().unwrap_or_default(),
                None => "",
            },
        );
        let section = match reservation.select_first("td[data-th=\"Standort (Printmedien)\"]") {
            Some(element) => element
                .text()
                .map(|x| x.trim())
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()[1..]
                .join(" "),
            None => String::new(),
        };
        let medium = Medium { ppn, title };
        let volume = Volume {
            medium,
            bar,
            signature,
            location: Location { library, section },
        };

        reservations.push(Reservation { volume, due_date });
    }
    ApiResponse {
        status: StatusCode::OK.as_u16(),
        body: Reservations(reservations),
    }
}

fn get_column_value(reservation: scraper::ElementRef, column: &str) -> String {
    match reservation.select_first(&format!("td[data-th={column}]")) {
        Some(element) => element.text().next().unwrap_or_default().trim().to_string(),
        None => String::new(),
    }
}

pub fn default_route() -> ApiResponse {
    let msg = "This route needs a session_token query parameter.".to_string();
    ApiResponse {
        status: StatusCode::BAD_REQUEST.as_u16(),
        body: Text(msg),
    }
}
