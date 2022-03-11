use crate::entities::participant::Participant;
use crate::entities::prize::Prize;
use crate::entities::promotion_result::PromotionResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Promotion {
    id: u64,
    name: String,
    description: String,
    prizes: Vec<Prize>,
    participants: Vec<Participant>,
}

impl Promotion {
    pub fn new(id: u64, promo: RawPromotion) -> Self {
        Promotion {
            id,
            name: promo.name,
            description: promo.description,
            prizes: vec![],
            participants: vec![],
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_name(&mut self, name: &str) {
        if !name.is_empty() {
            self.name = String::from(name);
        }
    }

    pub fn set_desc(&mut self, desc: &str) {
        self.description = String::from(desc);
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant)
    }

    pub fn delete_participant(&mut self, participant_id: u64) {
        let index = self
            .participants
            .iter()
            .enumerate()
            .find(|item| item.1.get_id() == participant_id)
            .map(|item| item.0)
            .unwrap();
        self.participants.remove(index);
    }

    pub fn add_prize(&mut self, prize: Prize) {
        self.prizes.push(prize)
    }

    pub fn delete_prize(&mut self, prize_id: u64) {
        let index = self
            .prizes
            .iter()
            .enumerate()
            .find(|item| item.1.get_id() == prize_id)
            .map(|item| item.0)
            .unwrap();
        self.prizes.remove(index);
    }

    pub fn raffle(&self) -> Vec<PromotionResult> {
        if self.prizes.len() == self.participants.len() {
            let prizes_and_participants = self
                .prizes
                .iter()
                .zip(self.participants.iter())
                .collect::<Vec<(_, _)>>();
            prizes_and_participants
                .iter()
                .map(|item| PromotionResult::new(item.1, item.0))
                .collect()
        } else {
            vec![]
        }
    }
}

#[derive(Deserialize)]
pub struct RawPromotion {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ShortPromotionInfo {
    id: u64,
    name: String,
    description: String,
}

impl From<&Promotion> for ShortPromotionInfo {
    fn from(promo: &Promotion) -> Self {
        ShortPromotionInfo {
            id: promo.id,
            name: promo.name.clone(),
            description: promo.description.clone(),
        }
    }
}