mod controllers;
mod models;
mod shared;

#[macro_use]
extern crate rocket;

use std::{collections::HashMap, sync::Arc};

use controllers::customer_controller;
use models::customer_model::Customer;
use rocket::{futures::lock::Mutex, State};

struct RocketContext {
    // pub db: Arc<PrismaClient>,
    pub customers: Arc<Mutex<HashMap<u8, Customer>>>,
}

type Ctx = State<RocketContext>;

// id   limite     saldo inicial
// 1    100000     0
// 2    80000      0
// 3    1000000    0
// 4    10000000   0
// 5    500000     0

#[launch]
async fn rocket() -> _ {
    let _customers: HashMap<u8, Customer> = HashMap::from_iter([
        (1, Customer::new(100_000)),
        (2, Customer::new(80_000)),
        (3, Customer::new(1_000_000)),
        (4, Customer::new(10_000_000)),
        (5, Customer::new(500_000)),
    ]);

    let customers = Arc::new(Mutex::new(_customers));

    rocket::build()
        .mount("/clientes", customer_controller::routes())
        .manage(RocketContext { customers })
}
