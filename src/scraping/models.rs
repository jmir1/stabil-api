#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Session {
    pub session_token: String,
    pub expiry: i64,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Medium {
    pub ppn: String,
    pub title: String,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Volume {
    pub medium: Medium,
    pub bar: String,
    pub signature: String,
    pub location: Location,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
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

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CheckedOut {
    pub volume: Volume,
    pub due_date: String,
    pub status: Status,
    pub renewals: i8,
    pub renewal_msg: String,
    pub warnings: i8,
    pub can_be_renewed: bool,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Reservation {
    pub volume: Volume,
    pub due_date: String,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Library {
    pub id: i32,
    pub name: String,
}

pub fn to_library(str: &str) -> Library {
    let name = str.to_string();
    match str {
        "Staats- und Universitätsbibliothek" => Library { id: 2, name },
        "FB Physik" => Library { id: 267, name },
        &_ => Library { id: 0, name },
    }
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct Location {
    pub library: Library,
    pub section: String,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct ApiResult<T: rocket_okapi::JsonSchema> {
    pub success: bool,
    pub data: T,
    pub msg: String,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct ApiResponse<T: rocket_okapi::JsonSchema> {
    pub status: u16,
    pub result: ApiResult<T>,
}

impl<'r, T: serde::Serialize + rocket_okapi::JsonSchema> rocket::response::Responder<'r, 'static>
    for ApiResponse<T>
{
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let string = serde_json::to_string(&self.result).unwrap();
        rocket::Response::build_from(string.respond_to(req)?)
            .header(rocket::http::ContentType::new("application", "json"))
            .status(rocket::http::Status::new(self.status))
            .ok()
    }
}

impl<T: serde::Serialize + rocket_okapi::JsonSchema> rocket_okapi::response::OpenApiResponderInner
    for ApiResponse<T>
{
    fn responses(
        gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<rocket_okapi::okapi::openapi3::Responses> {
        let mut responses = rocket_okapi::okapi::openapi3::Responses::default();
        let schema = gen.json_schema::<ApiResult<T>>();
        rocket_okapi::util::add_schema_response(&mut responses, 200, "application/json", schema)?;

        Ok(responses)
    }
}
