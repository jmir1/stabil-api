use crate::scraping::{
    models::{ApiResult, Loan, Medium},
    utils::{is_logged_in, Select},
};

#[get("/loans?<session_token>")]
pub async fn route(session_token: &str, client: &rocket::State<reqwest::Client>) -> String {
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
        let result = ApiResult::<Vec<Loan>> {
            success: false,
            data: vec![],
            msg: "session_token is invalid.".to_string(),
        };
        return serde_json::to_string(&result).unwrap();
    }

    let mut loans: Vec<Loan> = vec![];
    for loan in document.select_all("tr.myresearch-result") {
        let id = get_medium_id(loan.value().attr("id").unwrap());
        let title = get_column_value(loan, "Titel");
        let signature = get_column_value(loan, "Signatur");
        let due_date = get_column_value(loan, "Rückgabe");
        let renewals = get_column_value(loan, "Verlängerungen")
            .parse::<i8>()
            .unwrap();
        let warnings = get_column_value(loan, "Mahnungen").parse::<i8>().unwrap();
        let can_be_renewed = loan
            .select_all(&format!("td[data-th=Selection] > input[disabled]"))
            .len()
            == 0;
        let medium = Medium { id, title, signature };

        loans.push(Loan {
            medium,
            due_date,
            renewals,
            warnings,
            can_be_renewed,
        });
    }
    let result = ApiResult {
        success: true,
        data: loans,
        msg: String::new(),
    };
    serde_json::to_string(&result).unwrap()
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

#[get("/loans")]
pub fn default_route() -> String {
    let msg = "This route needs a session_token query parameter.".to_string();
    let result = ApiResult {
        success: false,
        data: String::new(),
        msg,
    };
    serde_json::to_string(&result).unwrap()
}
