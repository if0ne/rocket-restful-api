use crate::entities::participant::Participant;
use crate::entities::prize::Prize;
use serde::Serialize;

#[derive(Serialize)]
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
