use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Participant {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
pub struct RawParticipant {
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
