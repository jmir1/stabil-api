use axum::{extract::Query, http::StatusCode};
use yaz_rs::ZoomConnection;

use crate::scraping::models::{ApiResponse, ApiResult, BarcodeQuery};

#[utoipa::path(
    get,
    path = "/ppn_from_bar",
    responses(
        (status = 200, description = "Volume information", body = ApiResult<Option<i32>>),
        (status = 400, description = "Bad request", body = ApiResult<Option<i32>>),
        (status = 404, description = "Not found", body = ApiResult<Option<i32>>),
    )
)]
pub async fn route(query: Query<BarcodeQuery>) -> ApiResponse<Option<i32>> {
    let barcode = match query.barcode {
        Some(barcode) => barcode,
        _ => {
            return ApiResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "barcode query parameter is required.".to_string(),
                },
            }
        }
    };

    let server = "z3950.k10plus.de:210/opac-de-18";
    let mut connection = match ZoomConnection::connect(server) {
        Ok(connection) => connection,
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "Failed to connect to the library server.".to_string(),
                },
            };
        }
    };
    let _ = connection.option_set("preferredRecordSyntax", "usmarc");
    let mut resultset = match connection.search_pqf(&format!("@attr 1=8535 {}", barcode)) {
        Ok(resultset) => resultset,
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "Failed to perform search on the library server.".to_string(),
                },
            };
        }
    };
    let hit_count = resultset.size();
    if hit_count == 0 {
        return ApiResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            result: ApiResult {
                success: false,
                data: None,
                msg: "No volume found for the given barcode.".to_string(),
            },
        };
    }
    resultset.fetch(0, 1);
    let record = match resultset.record_text(0, "json") {
        Ok(Some(record)) => record,
        _ => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "Failed to retrieve record from the library server.".to_string(),
                },
            };
        }
    };
    let json = match serde_json::from_str::<serde_json::Value>(&record) {
        Ok(json) => json,
        Err(_) => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "Failed to parse record from the library server.".to_string(),
                },
            };
        }
    };
    let ppn = match json
        .get("fields")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("001"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .parse::<i32>()
    {
        Ok(ppn) => ppn,
        _ => {
            return ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                result: ApiResult {
                    success: false,
                    data: None,
                    msg: "Failed to extract PPN from the record.".to_string(),
                },
            }
        }
    };
    ApiResponse {
        status: StatusCode::OK.as_u16(),
        result: ApiResult {
            success: true,
            data: Some(ppn),
            msg: "PPN retrieved successfully.".to_string(),
        },
    }
}
