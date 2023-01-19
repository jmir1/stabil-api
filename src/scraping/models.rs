#[derive(serde::Serialize)]
pub struct Medium {
    pub id: String,
    pub title: String,
    pub signature: String,
}

#[derive(serde::Serialize)]
pub struct Loan {
    pub medium: Medium,
    pub due_date: String,
    pub renewals: i8,
    pub warnings: i8,
    pub can_be_renewed: bool,
}

#[derive(serde::Serialize)]
pub struct ApiResult<T: serde::Serialize> {
    pub success: bool,
    pub data: T,
    pub msg: String,
}
