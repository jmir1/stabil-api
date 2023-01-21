use crate::scraping::{
    models::{ApiResponse, ApiResult, CheckedOut, Medium},
    utils::{is_logged_in, Select},
};

#[rocket_okapi::openapi(tag = "Get a user's checked out items")]
#[get("/checked_out?<session_token>")]
pub async fn route(
    session_token: &str,
    client: &rocket::State<reqwest::Client>,
) -> ApiResponse<Vec<CheckedOut>> {
    let response_text = client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/Home")
        .header("cookie", format!("VUFIND_SESSION={session_token}"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&response_text);
    if !is_logged_in(&document) {
        let result = ApiResult::<Vec<CheckedOut>> {
            success: false,
            data: vec![],
            msg: "session_token is invalid.".to_string(),
        };
        return ApiResponse {
            status: rocket::http::Status::Unauthorized.code,
            result,
        };
    }

    let mut loans: Vec<CheckedOut> = vec![];
    for loan in document.select_all("tr.myresearch-result") {
        let id = get_medium_id(loan.value().attr("id").unwrap());
        let title = get_column_value(loan, "Titel");
        let mut location = vec![];
        for loc in loan.select_first("td[data-th=Titel").all_text() {
            let trimmed = loc.trim();
            if !trimmed.is_empty() {
                location.push(trimmed.to_string());
            }
        }
        let idx = location.iter().position(|x| x == "Ausleihstelle:").unwrap() + 1;
        location.drain(0..idx);
        let location = location.join(" ");
        let signature = get_column_value(loan, "Signatur");
        let due_date = get_column_value(loan, "Rückgabe");
        let renewals = get_column_value(loan, "Verlängerungen")
            .parse::<i8>()
            .unwrap();
        let warnings = get_column_value(loan, "Mahnungen").parse::<i8>().unwrap();
        let can_be_renewed = loan
            .select_all("td[data-th=Selection] > input[disabled]")
            .is_empty();
        let renew_id = if can_be_renewed {
            loan.select_first("td.checkbox > label > input[name=\"renewSelectedIDS[]\"]")
                .value()
                .attr("value")
                .unwrap()
                .to_string()
        } else {
            String::new()
        };
        let medium = Medium {
            id,
            title,
            signature,
            location,
        };

        loans.push(CheckedOut {
            medium,
            due_date,
            renewals,
            warnings,
            can_be_renewed,
            renew_id,
        });
    }
    let result = ApiResult {
        success: true,
        data: loans,
        msg: String::new(),
    };
    ApiResponse {
        status: rocket::http::Status::Ok.code,
        result,
    }
}

fn get_column_value(loan: scraper::ElementRef, column: &str) -> String {
    loan.select_first(&format!("td[data-th={column}]"))
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
#[get("/checked_out")]
pub fn default_route() -> ApiResponse<Vec<CheckedOut>> {
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
