use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Session {
    pub id: u32,
    pub name: String,
    // pub routeine_id: u32,
    // pub start_time: String,
    // pub end_time: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Sessions {
    pub value: Vec<Session>,
}

impl Sessions {
    pub fn add_session(&mut self, name: String) {
        let session = Session {
            id: self.next_id(),
            name,
        };
        self.value.push(session);
    }

    pub fn next_id(&self) -> u32 {
        self.value.len() as u32
    }
}
