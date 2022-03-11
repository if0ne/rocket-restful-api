use crate::database::PromoRepository;
use crate::entities::participant::RawParticipant;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, post};
use rocket_okapi::openapi;
use crate::routes::get_status;

///Добавление участника в промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Add participant to promo")]
#[post("/<id>/participant", format = "json", data = "<participant>")]
pub fn add_participant(
    db: &State<PromoRepository>,
    id: u64,
    participant: Json<RawParticipant>,
) -> (Status, Json<u64>) {
    get_status(db.add_participant(id, participant.0))
}

///Удаление участника из промоакцию
///id - идентификатор промоакции
#[openapi(tag = "Remove participant from promo")]
#[delete("/<promo_id>/participant/<participant_id>")]
pub fn delete_participant(db: &State<PromoRepository>, promo_id: u64, participant_id: u64) -> (Status, Json<()>) {
    get_status(db.delete_participant_from_promo(promo_id, participant_id))
}
