use serde_derive::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Quote {
    pub id: i64,
    pub quote: String,
    pub resilience: i64,
    pub acceptance: i64,
    pub wisdom: i64,
}

