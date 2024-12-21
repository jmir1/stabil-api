use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    scraping::{
        libraries::to_library,
        models::{to_status, ApiResponse, ApiResult, CheckedOut, Location, Medium, Volume},
        utils::{is_logged_in, Select},
    },
    SessionTokenQuery,
};

#[utoipa::path(
    get,
    path = "/checked_out",
    //params(GetCheckedOutQuery),
    responses(
        (status = 200, description = "Checked out items", body = ApiResult<Vec<CheckedOut>>),
        (status = 400, description = "Bad request", body = String),
        (status = 401, description = "Unauthorized", body = String),
    ),
    security(("session_token" = [])),
)]
#[worker::send]
pub async fn route(
    State(client): State<reqwest::Client>,
    query: Query<SessionTokenQuery>,
) -> ApiResponse<Vec<CheckedOut>> {
    let session_token = match &query.session_token {
        Some(token) => token,
        None => return default_route(),
    };
    let response_text = client
        .get("https://katalogplus.sub.uni-hamburg.de/vufind/MyResearch/Home")
        .header("cookie", format!("VUFIND_SESSION={}", session_token))
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
            status: StatusCode::UNAUTHORIZED.as_u16(),
            result,
        };
    }

    let mut loans: Vec<CheckedOut> = vec![];
    for loan in document.select_all("tr.myresearch-result") {
        let ppn = get_ppn(loan.value().attr("id").unwrap());
        let title = get_column_value(loan, "Titel");
        let status_string = loan
            .select_all("td[data-th=Titel] > div.alert-danger")
            .iter()
            .map(|x| x.text().next().unwrap_or(""))
            .next()
            .unwrap_or("");
        let status = to_status(status_string);
        let library_string = loan
            .select_all("td[data-th=Titel] > strong")
            .get(1)
            .unwrap()
            .text()
            .next()
            .unwrap_or("")
            .trim();
        let library = to_library(library_string);
        let section: Vec<&str> = loan
            .select_first("td[data-th=Titel]")
            .text()
            .map(|x| x.trim())
            .filter(|&x| !x.is_empty())
            .collect();
        let section_idx = section
            .iter()
            .position(|&x| x == "Ausleihstelle:")
            .unwrap_or(0)
            + 2;
        let section = section[section_idx..].join(" ");
        let signature = get_column_value(loan, "Signatur");
        let due_date = get_column_value(loan, "Rückgabe");
        let renewals = get_column_value(loan, "Verlängerungen")
            .parse::<i8>()
            .unwrap();
        let renewal_msg = loan
            .select_all("td[data-th=Verlängerungen] > div.alert-danger")
            .iter()
            .map(|x| x.text().next().unwrap_or(""))
            .next()
            .unwrap_or("")
            .to_string();
        let warnings = get_column_value(loan, "Mahnungen").parse::<i8>().unwrap();
        let can_be_renewed = loan
            .select_all("td[data-th=Selection] > input[disabled]")
            .is_empty();
        let volume_bar = if can_be_renewed {
            let uri = loan
                .select_first("td.checkbox > label > input[name=\"renewSelectedIDS[]\"]")
                .value()
                .attr("value")
                .unwrap()
                .to_string();
            get_bar(&uri)
        } else {
            String::new()
        };
        let location = Location { library, section };
        let medium = Medium { ppn, title };
        let volume = Volume {
            medium,
            bar: volume_bar,
            signature,
            location,
        };

        loans.push(CheckedOut {
            volume,
            due_date,
            status,
            renewals,
            renewal_msg,
            warnings,
            can_be_renewed,
        });
    }
    let result = ApiResult {
        success: true,
        data: loans,
        msg: String::new(),
    };
    ApiResponse {
        status: StatusCode::OK.as_u16(),
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

pub fn get_bar(bar_attr: &str) -> String {
    let start_idx = bar_attr.find(":bar:").unwrap() + 5;
    let end_idx = bar_attr.len();
    bar_attr[start_idx..end_idx].to_string()
}

fn get_ppn(ppn_attr: &str) -> String {
    let start_idx = ppn_attr.find(":ppn:").unwrap() + 5;
    let end_idx = ppn_attr.len();
    ppn_attr[start_idx..end_idx].to_string()
}

pub fn default_route() -> ApiResponse<Vec<CheckedOut>> {
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
