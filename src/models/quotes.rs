use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QuoteRequest {
    pub resilience: i32,
    pub acceptance: i32,
    pub wisdom: i32,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub resilience: i32,
    pub acceptance: i32,
    pub wisdom: i32,
}