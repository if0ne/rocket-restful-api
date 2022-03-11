use crate::database::PromoRepository;
use crate::entities::participant::RawParticipant;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, post};

#[post("/<id>/participant", format = "json", data = "<participant>")]
pub fn add_participant(
    db: &State<PromoRepository>,
    id: u64,
    participant: Json<RawParticipant>,
) -> Json<u64> {
    Json(db.add_participant(id, participant.0))
}

#[delete("/<promo_id>/participant/<participant_id>")]
pub fn delete_participant(db: &State<PromoRepository>, promo_id: u64, participant_id: u64) {
    db.delete_participant_from_promo(promo_id, participant_id);
}
