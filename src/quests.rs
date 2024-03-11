use crate::prelude::*;
//when making a new quest it'll always be false
#[derive(Clone, Debug)]
pub struct QuestDatabase {
    contents: HashMap<String, Quest>,
}
impl QuestDatabase {
    pub fn new(contents: HashMap<String, Quest>) -> Self {
        Self { contents }
    }
    // pub fn new quest(key: String) -> &Quest {

    // }
}
#[derive(Clone, Debug)]
pub struct Quest {
    is_complete: bool,
    // mutually_exclusive: bool,
    choices: HashMap<String, Quest>,
}

impl Quest {
    pub fn new(choices: HashMap<String, Quest>) -> Self {
        Self {
            is_complete: false,
            choices,
        }
    }
}
