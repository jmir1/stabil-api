use crate::scraping::{
    models::{to_library, ApiResponse, ApiResult, Location, Medium, Reservation},
    utils::{is_logged_in, Select},
};

#[rocket_okapi::openapi(tag = "Get a user's reserved items.")]
#[get("/reservations?<session_token>")]
pub async fn route(
    session_token: &str,
    client: &rocket::State<reqwest::Client>,
) -> ApiResponse<Vec<Reservation>> {
    let response_text = client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/Holds/List")
        .header("cookie", format!("VUFIND_SESSION={session_token}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&response_text);
    if !is_logged_in(&document) {
        let result = ApiResult::<Vec<Reservation>> {
            success: false,
            data: vec![],
            msg: "session_token is invalid.".to_string(),
        };
        return ApiResponse {
            status: rocket::http::Status::Unauthorized.code,
            result,
        };
    }

    let mut reservations: Vec<Reservation> = vec![];
    for reservation in document.select_all("tr.myresearch-result") {
        let id = get_medium_id(reservation.value().attr("id").unwrap());
        let title = reservation
            .select_first("td[data-th=Titel] > span.title")
            .text()
            .next()
            .unwrap()
            .to_string();
        let signature = get_column_value(reservation, "Signatur");
        let due_date = get_column_value(reservation, "Rückgabedatum");
        let cancel_id = reservation
            .select_first("td > label > input[name=\"cancelSelectedIDS[]\"]")
            .value()
            .attr("value")
            .unwrap()
            .to_string();
        let library = to_library(
            reservation
                .select_first("td[data-th=\"Standort (Printmedien)\"] > strong")
                .text()
                .next()
                .unwrap(),
        );
        let section = reservation
            .select_first("td[data-th=\"Standort (Printmedien)\"]")
            .text()
            .map(|x| x.trim())
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>()[1..]
            .join(" ");
        let medium = Medium {
            id,
            title,
            signature,
            location: Location { library, section },
        };

        reservations.push(Reservation {
            medium,
            due_date,
            cancel_id,
        });
    }
    let result = ApiResult {
        success: true,
        data: reservations,
        msg: String::new(),
    };
    ApiResponse {
        status: rocket::http::Status::Ok.code,
        result,
    }
}

fn get_column_value(reservation: scraper::ElementRef, column: &str) -> String {
    reservation
        .select_first(&format!("td[data-th={column}]"))
        .text()
        .next()
        .unwrap()
        .trim()
        .to_string()
}

fn get_medium_id(id_attr: &str) -> String {
    let start_idx = id_attr.find(":ppn:").unwrap() + 5;
    let end_idx = id_attr.len();
    id_attr[start_idx..end_idx].to_string()
}

#[rocket_okapi::openapi(skip)]
#[get("/reservations")]
pub fn default_route() -> ApiResponse<Vec<Reservation>> {
    let msg = "This route needs a session_token query parameter.".to_string();
    let result = ApiResult {
        success: false,
        data: vec![],
        msg,
    };
    ApiResponse {
        status: rocket::http::Status::BadRequest.code,
        result,
    }
}
