use crate::database::PromoRepository;
use crate::entities::prize::RawPrize;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, post};
use rocket::http::Status;
use rocket_okapi::openapi;
use crate::routes::get_status;

///Добавление приза в промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Prize")]
#[post("/<id>/prize", format = "json", data = "<prize>")]
pub fn add_prize(db: &State<PromoRepository>, id: u64, prize: Json<RawPrize>) -> (Status, Json<u64>) {
    get_status(db.add_prize(id, prize.0))
}

///Удаление приза из промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Prize")]
#[delete("/<promo_id>/prize/<prize_id>")]
pub fn delete_prize(db: &State<PromoRepository>, promo_id: u64, prize_id: u64) -> (Status, Json<()>) {
    get_status(db.delete_prize_from_promo(promo_id, prize_id))
}
