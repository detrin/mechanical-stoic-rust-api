pub async fn route_info() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "msg": "Wellcome to the Mechanical Stoic API!",
    }))
}