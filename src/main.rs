mod controllers;
mod models;
mod repository;
mod shared;

#[macro_use]
extern crate rocket;

use std::sync::Arc;

use controllers::customer_controller;
use db::PrismaClient;
use rocket::State;

struct RocketContext {
    pub db: Arc<PrismaClient>,
}

type Ctx = State<RocketContext>;

// id   limite     saldo inicial
// 1    100000     0
// 2    80000      0
// 3    1000000    0
// 4    10000000   0
// 5    500000     0
async fn initialize_db(db: &Arc<PrismaClient>) {
    let _ = db.customer().delete_many(vec![]).exec().await;
    let _ = db.transaction().delete_many(vec![]).exec().await;

    let data: Vec<(i32, i32, Vec<db::customer::SetParam>)> = vec![
        (0, 100_000, vec![db::customer::id::set(1)]),
        (0, 80_000, vec![db::customer::id::set(2)]),
        (0, 1_000_000, vec![db::customer::id::set(3)]),
        (0, 10_000_000, vec![db::customer::id::set(4)]),
        (0, 5_00_000, vec![db::customer::id::set(5)]),
    ];

    let _ = db.customer().create_many(data).exec().await;
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    initialize_db(&db).await;

    rocket::build()
        .mount("/clientes", customer_controller::routes())
        .manage(RocketContext { db })
}
