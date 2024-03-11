use crate::prelude::*;
//when making a new quest it'll always be false
pub type QuestDatabase = HashMap<String, Quest>;
#[derive(Clone, Debug)]
pub struct Quest {
    is_complete: bool,
    mutually_exclusive: bool,
    choices: HashMap<String, Quest>,
}
