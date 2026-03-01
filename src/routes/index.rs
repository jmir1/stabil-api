#[utoipa::path(
    operation_id = "get_index",
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello there!", body = String)
    )
)]
pub async fn get() -> &'static str {
    "Hello World!"
}
