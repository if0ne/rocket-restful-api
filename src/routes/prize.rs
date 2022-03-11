use crate::database::PromoRepository;
use crate::entities::prize::RawPrize;
use rocket_okapi::openapi;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, post};

///Добавление приза в промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Add prize to promo")]
#[post("/<id>/prize", format = "json", data = "<prize>")]
pub fn add_prize(db: &State<PromoRepository>, id: u64, prize: Json<RawPrize>) -> Json<u64> {
    Json(db.add_prize(id, prize.0))
}

///Удаление приза из промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Remove participant to promo")]
#[delete("/<promo_id>/prize/<prize_id>")]
pub fn delete_prize(db: &State<PromoRepository>, promo_id: u64, prize_id: u64) {
    db.delete_prize_from_promo(promo_id, prize_id);
}
