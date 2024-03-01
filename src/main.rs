mod controllers;
mod models;
mod repository;
mod shared;

#[macro_use]
extern crate rocket;

use std::{env, sync::Arc};

use controllers::customer_controller;
use dotenv::dotenv;
use models::customer_model::Customer;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use rocket::State;

struct RocketContext {
    pub db: DBClient,
}

pub type DBClient = Arc<Client>;

type Ctx = State<RocketContext>;

// id   limite     saldo inicial
// 1    100000     0
// 2    80000      0
// 3    1000000    0
// 4    10000000   0
// 5    500000     0

async fn initialize_db(db: &DBClient) -> Result<(), Box<dyn std::error::Error>> {
    db.database("rinha").drop(None).await?;

    let customers = vec![
        Customer {
            id: 1,
            limit: 100_000,
            ..Customer::default()
        },
        Customer {
            id: 2,
            limit: 80_000,
            ..Customer::default()
        },
        Customer {
            id: 3,
            limit: 1_000_000,
            ..Customer::default()
        },
        Customer {
            id: 4,
            limit: 10_000_000,
            ..Customer::default()
        },
        Customer {
            id: 5,
            limit: 500_000,
            // transactions: vec![Transaction {
            //     created_at: Utc::now(),
            //     description: Description("asd".to_string()),
            //     transaction_type: TransactionType::Credit,
            //     value: 1,
            // }],
            ..Customer::default()
        },
    ];

    for customer in customers {
        db.database("rinha")
            .collection::<Customer>("customers")
            .insert_one(customer, None)
            .await?;
    }

    Ok(())
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .expect("ClientOptions error!");

    let client = Client::with_options(options).expect("Client::with_options error!");

    let db = Arc::new(client);

    initialize_db(&db).await.expect("Error initialize_db");

    rocket::build()
        .mount("/clientes", customer_controller::routes())
        .manage(RocketContext { db })
}
