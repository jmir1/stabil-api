#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello there!", body = String)
    )
)]
pub async fn route() -> &'static str {
    "Hello World!"
}
