use chrono::Utc;
use rocket::{http::Status, serde::json::Json, Route};
use serde_json::json;

use crate::{
    models::transaction_model::{Transaction, TransactionType},
    repository::customer_repository::CustomerRepository,
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

    let customer = repo.get_one(&id).await?;

    if customer.is_none() {
        return Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        )));
    }

    let mut customer = customer.unwrap();

    let transaction = transaction_json.0;

    match transaction.transaction_type {
        TransactionType::Credit => {
            customer.balance += transaction.value;
            customer.transactions.push(transaction);
        }
        TransactionType::Debit => {
            if customer.balance + customer.limit >= transaction.value {
                customer.balance -= transaction.value;
                customer.transactions.push(transaction);
            } else {
                return Ok(ApiResponse::Error((
                    Status::UnprocessableEntity,
                    "Não tem limite suficiente".to_string(),
                )));
            }
        }
    }

    repo.update(&customer.id, &customer).await?;

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

    let customer = repo.get_one(&id).await?;

    if customer.is_none() {
        return Ok(ApiResponse::Error((
            Status::NotFound,
            "Cliente não encontrado".to_string(),
        )));
    }

    let customer = customer.unwrap();

    Ok(ApiResponse::Success(
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
    ))
}
