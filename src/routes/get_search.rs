use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    scraping::{
        models::{ApiResponse, ApiResult, Medium},
        utils::{get_medium_ppn_from_href, Select},
    },
    SearchQuery,
};


#[utoipa::path(
    get,
    path = "/search",
    responses(
        (status = 200, description = "Search results", body = ApiResult<Vec<Medium>>),
        (status = 400, description = "Bad request", body = String),
    ),
    params(
        ("query" = String, Query, description = "Query string for the search"),
    ),
)]
#[worker::send]
pub async fn route(
    State(state): State<crate::State>,
    query: Query<SearchQuery>,
) -> ApiResponse<Vec<Medium>> {
    let query_string = match &query.query {
        Some(query) => query,
        None => "",
    };
    let page = match &query.page {
        Some(page) => page.to_string(),
        None => "1".to_string(),
    };
    let response_text = match state
        .client
        .get(format!("https://katalogplus.sub.uni-hamburg.de/vufind/Search/Results?lookfor={query_string}&page={page}&type=AllFields&searchbox=1"))
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

    let mut media: Vec<Medium> = vec![];
    for medium in document.select_all("div[id^=result] a.title") {
        let ppn = get_medium_ppn_from_href(medium.value().attr("href").unwrap_or_default());
        let title = medium.text().next().unwrap_or_default().trim().to_string();
        let medium = Medium { ppn, title };
        media.push(medium);
    }
    let result = ApiResult {
        success: true,
        data: media,
        msg: String::new(),
    };
    ApiResponse {
        status: StatusCode::OK.as_u16(),
        result,
    }
}

