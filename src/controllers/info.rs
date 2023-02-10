pub async fn route_info() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "routes": ["/", "/register", "/payment", "/recover"],
        "routes_info": {
            "/" : "this route",
            "/register": "register by entering your pubkey, backup and ln payment",
            "/login/:pubkey": "request a payment to register your pubkey with",
            "/recover": "recover your backup by entering your pubkey that was registered with it",
        }
    }))
}
