use crate::database::PromoRepository;
use crate::entities::prize::RawPrize;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, post};

#[post("/<id>/prize", format = "json", data = "<prize>")]
pub fn add_prize(db: &State<PromoRepository>, id: u64, prize: Json<RawPrize>) -> Json<u64> {
    Json(db.add_prize(id, prize.0))
}

#[delete("/<promo_id>/prize/<prize_id>")]
pub fn delete_prize(db: &State<PromoRepository>, promo_id: u64, prize_id: u64) {
    db.delete_prize_from_promo(promo_id, prize_id);
}
