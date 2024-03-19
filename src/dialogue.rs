use crate::prelude::*;

pub struct DialogueDatabase {
    contents: HashMap<String, DialogueTree>,
}

pub struct DialogueTree {
    nodes: HashMap<String, DialogueNode>,
}

pub struct DialogueNode {
    npc_text: String,     //what the npc is saying in response to the previous choice
    choices: vec<String>, //a collection of keys pointing to
}
