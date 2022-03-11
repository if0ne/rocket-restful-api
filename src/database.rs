use crate::entities::participant::{Participant, RawParticipant};
use crate::entities::prize::{Prize, RawPrize};
use crate::entities::promotion::{Promotion, RawPromotion};
use crate::entities::promotion_result::PromotionResult;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct PromoRepository {
    promotions: Arc<Mutex<Vec<Promotion>>>,
    promotion_counter: AtomicU64,

    prizes_counter: AtomicU64,
    participants_counter: AtomicU64,
}

impl PromoRepository {
    pub fn new() -> Self {
        PromoRepository {
            promotions: Arc::new(Mutex::new(vec![])),
            promotion_counter: AtomicU64::new(0),
            prizes_counter: AtomicU64::new(0),
            participants_counter: AtomicU64::new(0),
        }
    }

    pub fn insert_promo(&self, promo: RawPromotion) -> u64 {
        let id = self.promotion_counter.fetch_add(1, Ordering::SeqCst);
        self.promotions
            .lock()
            .unwrap()
            .push(Promotion::new(id, promo));
        id
    }

    pub fn get_all_promo(&self) -> Arc<Mutex<Vec<Promotion>>> {
        self.promotions.clone()
    }

    pub fn get_promo_by_id(&self, id: u64) -> Result<Promotion, ()> {
        let item = self
            .promotions
            .lock()
            .unwrap()
            .iter()
            .find(|item| item.get_id() == id)
            .cloned();
        item.ok_or(())
    }

    pub fn edit_promo_by_id(&self, id: u64, new_info: &RawPromotion) -> Result<(), ()> {
        let mut promos = self.promotions.lock().unwrap();

        let item = promos
            .iter_mut()
            .find(|item| item.get_id() == id)
            .ok_or(())?;

        item.set_name(&new_info.name);
        item.set_desc(&new_info.description);

        Ok(())
    }

    pub fn delete_promo_by_id(&self, id: u64) -> Result<(), ()> {
        let promos = self.promotions.lock().unwrap();
        let index = promos
            .iter()
            .enumerate()
            .find(|item| item.1.get_id() == id)
            .ok_or(())?;

        self.promotions.lock().unwrap().remove(index.0);

        Ok(())
    }

    pub fn add_participant(&self, promo_id: u64, participant: RawParticipant) -> Result<u64, ()> {
        let id = self.participants_counter.fetch_add(1, Ordering::SeqCst);
        let participant = Participant::new(id, participant);

        let mut promos = self.promotions.lock().unwrap();

        let promo = promos
            .iter_mut()
            .find(|item| item.get_id() == promo_id)
            .ok_or(())?;

        promo.add_participant(participant);

        Ok(id)
    }

    pub fn delete_participant_from_promo(
        &self,
        promo_id: u64,
        participant_id: u64,
    ) -> Result<(), ()> {
        let mut promos = self.promotions.lock().unwrap();

        let promo = promos
            .iter_mut()
            .find(|item| item.get_id() == promo_id)
            .ok_or(())?;

        promo.delete_participant(participant_id)
    }

    pub fn add_prize(&self, promo_id: u64, prize: RawPrize) -> Result<u64, ()> {
        let id = self.prizes_counter.fetch_add(1, Ordering::SeqCst);
        let prize = Prize::new(id, prize);

        let mut promos = self.promotions.lock().unwrap();

        let promo = promos
            .iter_mut()
            .find(|item| item.get_id() == promo_id)
            .ok_or(())?;

        promo.add_prize(prize);

        Ok(id)
    }

    pub fn delete_prize_from_promo(&self, promo_id: u64, prize_id: u64) -> Result<(), ()> {
        let mut promos = self.promotions.lock().unwrap();

        let promo = promos
            .iter_mut()
            .find(|item| item.get_id() == promo_id)
            .ok_or(())?;

        promo.delete_prize(prize_id)
    }

    pub fn raffle_promo(&self, promo_id: u64) -> Result<Vec<PromotionResult>, ()> {
        let promos = self.promotions.lock().unwrap();

        let promo = promos
            .iter()
            .find(|item| item.get_id() == promo_id)
            .ok_or(())?;

        promo.raffle()
    }
}
