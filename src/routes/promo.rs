use crate::database::PromoRepository;
use crate::entities::promotion::{Promotion, RawPromotion, ShortPromotionInfo};
use crate::entities::promotion_result::PromotionResult;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, get, post, put};

#[post("/", format = "json", data = "<promo>")]
pub fn add_promo(db: &State<PromoRepository>, promo: Json<RawPromotion>) -> Json<u64> {
    Json(db.insert_promo(promo.0))
}

#[get("/")]
pub fn get_all_promo(db: &State<PromoRepository>) -> Json<Vec<ShortPromotionInfo>> {
    let all_promo = db.get_all_promo();
    let short_info = all_promo
        .lock()
        .unwrap()
        .iter()
        .map(|promo| promo.into())
        .collect::<Vec<ShortPromotionInfo>>();
    Json(short_info)
}

#[get("/<id>")]
pub fn get_promo(db: &State<PromoRepository>, id: u64) -> Json<Promotion> {
    let promo = db.get_promo_by_id(id);
    Json(promo.unwrap())
}

#[put("/<id>", format = "json", data = "<promo>")]
pub fn edit_promo(db: &State<PromoRepository>, id: u64, promo: Json<RawPromotion>) {
    db.edit_promo_by_id(id, &promo.0)
}

#[delete("/<id>")]
pub fn delete_promo(db: &State<PromoRepository>, id: u64) {
    db.delete_promo_by_id(id)
}

#[post("/<id>/raffle")]
pub fn raffle_promo(db: &State<PromoRepository>, id: u64) -> Json<Vec<PromotionResult>> {
    let result = db.raffle_promo(id);
    Json(result)
}