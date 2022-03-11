use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Участник промоакции
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    ///Идентификатор
    id: u64,
    ///Имя
    name: String,
}

///Информация о новом участнике
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RawParticipant {
    ///Имя
    pub name: String,
}

impl Participant {
    pub fn new(id: u64, participant: RawParticipant) -> Self {
        Participant {
            id,
            name: participant.name,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
