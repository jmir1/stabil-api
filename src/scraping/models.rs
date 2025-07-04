use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Session {
    pub session_token: String,
    pub expiry: i64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Medium {
    pub ppn: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Volume {
    pub medium: Medium,
    pub bar: String,
    pub signature: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    PickupRack,
    Default,
    Other,
}

pub fn to_status(str: &str) -> Status {
    match str {
        "Ausleihstatus: Abholregal" => Status::PickupRack,
        "" => Status::Default,
        &_ => Status::Other,
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CheckedOut {
    pub volume: Volume,
    pub due_date: String,
    pub status: Status,
    pub renewals: i8,
    pub renewal_msg: String,
    pub warnings: i8,
    pub can_be_renewed: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Reservation {
    pub volume: Volume,
    pub due_date: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Library {
    pub id: i32,
    pub name: String,
    pub filter: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Location {
    pub library: Library,
    pub section: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResult<T> {
    pub success: bool,
    pub data: T,
    pub msg: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub result: ApiResult<T>,
}

impl<T: Serialize> axum::response::IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = serde_json::to_string(&self.result).unwrap_or_default();
        axum::http::Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body))
            .unwrap_or_default()
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, IntoParams)]
pub struct SessionTokenQuery {
    #[serde_as(as = "NoneAsEmptyString")]
    pub session_token: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, IntoParams)]
pub struct SearchQuery {
    #[serde_as(as = "NoneAsEmptyString")]
    pub query: Option<String>,
    pub page: Option<i32>,
}

#[serde_as]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReservationData {
    pub ppn: String,
    pub doc_id: String,
    pub item_id: String,
    pub hash_key: String,
}
