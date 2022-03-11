mod database;
mod entities;
mod routes;

use crate::database::PromoRepository;
use rocket::http::Method;
use rocket::{Config};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use crate::routes::participant::{add_participant, delete_participant};
use crate::routes::prize::{add_prize, delete_prize};
use crate::routes::promo::{
    add_promo, delete_promo, edit_promo, get_all_promo, get_promo, raffle_promo,
};

use crate::routes::promo::{
    okapi_add_operation_for_add_promo_,
    okapi_add_operation_for_get_all_promo_,
    okapi_add_operation_for_get_promo_,
    okapi_add_operation_for_edit_promo_,
    okapi_add_operation_for_delete_promo_,
    okapi_add_operation_for_raffle_promo_,
};

use crate::routes::participant::{
    okapi_add_operation_for_add_participant_,
    okapi_add_operation_for_delete_participant_,
};

use crate::routes::prize::{
    okapi_add_operation_for_add_prize_,
    okapi_add_operation_for_delete_prize_,
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
            openapi_get_routes![
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
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../promo/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await
        .unwrap();
}
