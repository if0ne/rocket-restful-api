use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Prize {
    id: u64,
    description: String,
}

#[derive(Deserialize)]
pub struct RawPrize {
    pub name: String,
}
impl Prize {
    pub fn new(id: u64, promo: RawPrize) -> Self {
        Prize {
            id,
            description: promo.name,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
