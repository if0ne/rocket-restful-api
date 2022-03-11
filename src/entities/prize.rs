use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Приз
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Prize {
    ///Идентификатор
    id: u64,
    ///Описание
    description: String,
}

///Информация о новом призе
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RawPrize {
    ///Описание
    pub description: String,
}
impl Prize {
    pub fn new(id: u64, promo: RawPrize) -> Self {
        Prize {
            id,
            description: promo.description,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
