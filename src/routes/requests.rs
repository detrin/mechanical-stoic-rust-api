use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuoteRequest {
    pub resilience: i64,
    pub acceptance: i64,
    pub wisdom: i64,
}