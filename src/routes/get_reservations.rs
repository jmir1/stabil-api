use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    scraping::{
        libraries::to_library,
        models::{ApiResponse, ApiResult, Location, Medium, Reservation, Volume},
        utils::{is_logged_in, Select},
    },
    SessionTokenQuery,
};

use super::get_checked_out::get_bar;

#[utoipa::path(
    get,
    path = "/reservations",
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
) -> ApiResponse<Vec<Reservation>> {
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
                result: ApiResult {
                    success: false,
                    data: vec![],
                    msg: "Failed to connect to the library server.".to_string(),
                },
            };
        }
        Ok(response) => response.text().await.unwrap_or_default(),
    };
    let document = scraper::Html::parse_document(&response_text);
    if !is_logged_in(&document) {
        let result = ApiResult::<Vec<Reservation>> {
            success: false,
            data: vec![],
            msg: "session_token is invalid.".to_string(),
        };
        return ApiResponse {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            result,
        };
    }

    let mut reservations: Vec<Reservation> = vec![];
    for reservation in document.select_all("tr.myresearch-result") {
        let ppn = get_medium_ppn(reservation.value().attr("id").unwrap_or_default());
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
    let result = ApiResult {
        success: true,
        data: reservations,
        msg: String::new(),
    };
    ApiResponse {
        status: StatusCode::OK.as_u16(),
        result,
    }
}

fn get_column_value(reservation: scraper::ElementRef, column: &str) -> String {
    match reservation.select_first(&format!("td[data-th={column}]")) {
        Some(element) => element.text().next().unwrap_or_default().trim().to_string(),
        None => String::new(),
    }
}

fn get_medium_ppn(id_attr: &str) -> String {
    let start_idx = id_attr.find(":ppn:").unwrap_or_default() + 5;
    let end_idx = id_attr.len();
    id_attr[start_idx..end_idx].to_string()
}

pub fn default_route() -> ApiResponse<Vec<Reservation>> {
    let msg = "This route needs a session_token query parameter.".to_string();
    let result = ApiResult {
        success: false,
        data: vec![],
        msg,
    };
    ApiResponse {
        status: StatusCode::BAD_REQUEST.as_u16(),
        result,
    }
}
