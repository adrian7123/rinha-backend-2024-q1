use std::sync::Arc;

#[macro_use]
extern crate rocket;

use db::PrismaClient;
use rocket::State;

struct RocketContext {
    db: Arc<PrismaClient>,
}

type Ctx = State<RocketContext>;

#[get("/")]
pub fn hello() -> &'static str {
    "Olá, Mundo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
