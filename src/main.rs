mod database;
mod entities;
mod routes;

use crate::database::PromoRepository;
use rocket::http::Method;
use rocket::{routes, Config};
use rocket_cors::{AllowedOrigins, CorsOptions};

use crate::routes::participant::{add_participant, delete_participant};
use crate::routes::prize::{add_prize, delete_prize};
use crate::routes::promo::{
    add_promo, delete_promo, edit_promo, get_all_promo, get_promo, raffle_promo,
};

#[rocket::main]
async fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Delete, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let config = Config::figment().merge(("port", 8080));

    rocket::custom(config)
        .attach(cors.to_cors().unwrap())
        .manage(PromoRepository::new())
        .mount(
            "/promo",
            routes![
                add_promo,
                get_all_promo,
                get_promo,
                edit_promo,
                delete_promo,
                add_participant,
                delete_participant,
                raffle_promo,
                add_prize,
                delete_prize,
            ],
        )
        .launch()
        .await
        .unwrap();
}
