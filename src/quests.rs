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
    pub fn quest(&mut self, key: &str) -> &Quest {
        self.contents
            .get(key)
            .expect("Invalid key caused failure to retrieve quest.")
    }
}
#[derive(Clone, Debug)]
pub struct Quest {
    is_complete: bool,
    // mutually_exclusive: bool,
    choices: Vec<(String, Quest)>,
}

impl Quest {
    pub fn new(choices: Vec<(String, Quest)>) -> Self {
        Self {
            is_complete: false,
            choices,
        }
    }
    // pub fn new_
    pub fn step(&mut self, step_name: &str) -> Option<&Quest> {
        for (choicename, questnode) in self.choices.iter() {
            if step_name == choicename {
                return Some(questnode);
            } else {
                return None;
            }
        }
        return None;
    }
    pub fn flag_complete(&mut self) {
        self.is_complete = true;
    }
    pub fn status(&self) -> bool {
        self.is_complete
    }
}
