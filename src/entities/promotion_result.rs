use crate::entities::participant::Participant;
use crate::entities::prize::Prize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PromotionResult {
    winner: Participant,
    prize: Prize,
}

impl PromotionResult {
    pub fn new(winner: &Participant, prize: &Prize) -> Self {
        PromotionResult {
            winner: winner.clone(),
            prize: prize.clone(),
        }
    }
}
