use chrono::Utc;
use db;
use rocket::{http::Status, serde::json::Json, Route};
use serde_json::json;

use crate::{
    models::transaction_model::Transaction,
    repository::{
        customer_repository::CustomerRepository, transaction_repository::TransactionRepository,
    },
    shared::api::{ApiResponse, ApiResult},
    Ctx,
};

pub fn routes() -> Vec<Route> {
    routes![customer_create_transaction, customer_extract]
}

#[post("/<id>/transacoes", data = "<transaction_json>")]
async fn customer_create_transaction(
    ctx: &Ctx,
    id: i32,
    transaction_json: Json<Transaction>,
) -> ApiResult {
    let repo = CustomerRepository::new(&ctx.db);
    let transaction_repo = TransactionRepository::new(&ctx.db);

    let customer = repo.get_one(id).await?;

    if customer.is_none() {
        return Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        )));
    }

    let mut customer = customer.unwrap();

    match customer.transact(transaction_json.0.clone()) {
        Ok(c) => customer = c,
        Err(msg) => return Ok(ApiResponse::Error((Status::UnprocessableEntity, msg))),
    }

    // cria transaction
    transaction_repo
        .create(transaction_json.0, customer.id.clone())
        .await?;

    // atualiza o cliente
    repo.update(
        customer.id.clone(),
        vec![
            db::customer::balance::set(customer.balance.clone()),
            db::customer::limit::set(customer.limit.clone()),
        ],
    )
    .await?;

    Ok(ApiResponse::Success(
        json!({
            "limite": customer.limit,
            "saldo": customer.balance
        })
        .to_string(),
    ))
}

#[get("/<id>/extrato")]
async fn customer_extract(ctx: &Ctx, id: i32) -> ApiResult {
    let repo = CustomerRepository::new(&ctx.db);

    match repo.get_one(id).await {
        Ok(Some(customer)) => Ok(ApiResponse::Success(
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
        Ok(None) => Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        ))),
        Err(_) => Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        ))),
    }
}
