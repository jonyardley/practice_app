use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Exersize {
    pub id: u32,
    pub name: String,
    // pub tags: Vec<Tag>,
    // pub variants: Vec<Variant>,
}

// #[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
// pub struct Variant {
//     pub name: String,
// }

// #[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
// pub struct Tag {
//     pub name: String,
// }

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Exersizes {
    pub value: Vec<Exersize>,
}

impl Exersizes {
    pub fn add_exersize(&mut self, name: String) {
        let exersize = Exersize {
            id: self.next_id(),
            name,
        };
        self.value.push(exersize);
    }

    pub fn next_id(&self) -> u32 {
        self.value.len() as u32
    }
}
