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
    pub fn quest(&mut self, key: &str) -> Quest {
        self.contents
            .get(key)
            .expect("Invalid key caused failure to retrieve quest.")
            .clone()
    }
    //make update quest function
    pub fn update(&mut self, key: &str, value: Quest) {
        self.contents.insert(key.to_string(), value);
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

    pub fn step(&self, step_name: &str) -> Option<&Quest> {
        for (choicename, questnode) in self.choices.iter() {
            if step_name == choicename {
                return Some(questnode);
            } else {
                return None;
            }
        }
        return None;
    }

    pub fn step_mut(&mut self, step_name: &str) -> Option<&mut Quest> {
     for (choicename, mut questnode) in self.choices.iter() {
            if step_name == choicename {
                return Some(&mut questnode);
            } else {
                return None;
            }
        }
        return None;
    }
    pub fn complete(&mut self) {
        self.is_complete = true;
    }
    pub fn status(&self) -> bool {
        self.is_complete
    }
}
