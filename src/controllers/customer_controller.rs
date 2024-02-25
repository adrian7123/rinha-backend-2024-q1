use chrono::Utc;
use rocket::{http::Status, serde::json::Json, Route};
use serde_json::json;

use crate::{
    models::transaction_model::Transaction,
    shared::api::{ApiResponse, ApiResult},
    Ctx,
};

pub fn routes() -> Vec<Route> {
    routes![customer_create_transaction, customer_extract]
}

#[post("/<id>/transacoes", data = "<transaction>")]
async fn customer_create_transaction(
    ctx: &Ctx,
    id: u8,
    transaction: Json<Transaction>,
) -> ApiResult {
    match ctx.customers.lock().await.get_mut(&id) {
        Some(customer) => match customer.transact(transaction.0) {
            Ok(()) => Ok(ApiResponse::Success(
                json!({
                    "limite": customer.limit,
                    "saldo": customer.balance
                })
                .to_string(),
            )),
            Err(msg) => Ok(ApiResponse::Error((Status::UnprocessableEntity, msg))),
        },
        None => Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        ))),
    }
}

#[get("/<id>/extrato")]
async fn customer_extract(ctx: &Ctx, id: u8) -> ApiResult {
    match ctx.customers.lock().await.get(&id) {
        Some(customer) => Ok(ApiResponse::Success(
            json!(
                {
                    "saldo": {
                        "total": customer.balance,
                        "data_extrato": Utc::now(),
                        "limite": customer.limit
                    },
                    "ultimas_transacoes": customer.transactions,
                }
            )
            .to_string(),
        )),
        None => Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        ))),
    }
}
