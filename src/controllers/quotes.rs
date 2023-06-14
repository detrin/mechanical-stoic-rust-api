use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::{quotes::Quote, quotes::QuoteRequest};

pub async fn get_random_quote(
    Json(quote_request): Json<QuoteRequest>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {

    if quote_request.resilience < 1 || quote_request.resilience > 100 {
        return Err(AppError::ResilienceOutOfRange);
    }
    if quote_request.acceptance < 1 || quote_request.acceptance > 100 {
        return Err(AppError::AcceptanceOutOfRange);
    }
    if quote_request.wisdom < 1 || quote_request.wisdom > 100 {
        return Err(AppError::WisdomOutOfRange);
    }

    let query = "
        SELECT *
        FROM (
            SELECT *, ABS(resilience - $1) + ABS(acceptance - $2) + ABS(wisdom - $3) AS distance
            FROM mechanical_stoic.quotes
        ) AS subquery
        WHERE distance = (
            SELECT MIN(distance)
            FROM (
                SELECT ABS(resilience - $4) + ABS(acceptance - $5) + ABS(wisdom - $6) AS distance
                FROM mechanical_stoic.quotes
            ) AS subquery2
        )
        ORDER BY RANDOM()
        LIMIT 1;
    ";

    let quote = sqlx::query_as::<_, Quote>(query)
        .bind(quote_request.resilience)
        .bind(quote_request.acceptance)
        .bind(quote_request.wisdom)
        .bind(quote_request.resilience)
        .bind(quote_request.acceptance)
        .bind(quote_request.wisdom)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;
    

    let quote = json!({
        "quote": quote.quote,
        "resilience": quote.resilience,
        "acceptance": quote.acceptance,
        "wisdom": quote.wisdom,
    });
    // print the quote to the console
    // println!("{}", quote);
    Ok(Json(quote))

}