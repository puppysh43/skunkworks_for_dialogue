use crate::prelude::*;
/* The interaction menu will be a simple menu where the player can view multiple
options and select an entry.
- Each entry will be able to be viewed or not depending on conditions set by each entry
(for example, having a certain skill level or perk)
- it will have a string containing what is initially displayed in the menu
ex - "[strength check] try to push boulder"
- it can contain some kind of function that either produces a bool or maybe a result enum
(failure, partial success, full success)
- it can also not require one at all
- it will then have results that it will act on/a single function it will execute (will need to work out how to make borrowchecker here happy.)
- this will then print something to the screen/UI
- before going back to the menu

*/

///This is the root level data structure that contains all Interaction Menus for
///every entity in yr game. To play nice with the borrowchecker it will never
///give you direct access to them after being constructed and will only give you
///a clone of a specific menu
pub struct InteractionMenuDatabase {
    contents: HashMap<String, InteractionMenu>,
}
impl InteractionMenuDatabase {
    pub fn new(contents: HashMap<String, InteractionMenu>) -> Self {
        Self { contents }
    }
    pub fn get_interaction_menu(&self, key: String) -> Option<InteractionMenu> {
        if self.contents.contains_key(&key) == true {
            let int_menu = self.contents.get(&key).unwrap().clone();
            Some(int_menu)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
///A single tiered interaction menu for basic player interactions with the environment
///that isn't dialogue. Ex. a boulder blocking a doorway that can be pushed w/ a str
///check, destroyed with an explosive item, or moved if you have NPC help.
pub struct InteractionMenu {
    header: String, //this is the string displayed that describes the object being interacted with
    pub options: Vec<IntMenuEntry>,
}
impl InteractionMenu {
    pub fn new(header: String, options: Vec<IntMenuEntry>) -> Self {
        InteractionMenu { header, options }
    }
    pub fn new_blank(header: String) -> Self {
        InteractionMenu {
            header,
            options: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, entry: IntMenuEntry) {
        self.options.push(entry);
    }
    pub fn get_entry(&self, index: usize) -> IntMenuEntry {
        self.options[index].clone()
    }
}
#[derive(Clone, Debug)]
///This is the base datatype for every node. It contains the text shown when displaying
///all options in the interaction menu, optional visibility conditions when at the root
///of the menu, optional checks and consequences, and the different strings that will
///be displayed depending on the result of aformentioned checks and consequences.
pub struct IntMenuEntry {
    ///The text displayed on the root level of the interaction menu describing what the
    ///option actually is (ex. "[Strength Check] Push The Boulder")
    entry_text: String,
    ///Each entry has the option of conditional visibility, meaning it will only be displayed if
    ///certain conditions are met. This can prevent metagaming/be more "realistic", preventing
    ///players from seeing that they can use items they're supposed to find organically or
    ///options only available after a related quest has been completed.
    vis_condition: Option<VisCondition>,
    ///Each entry has the option of being able to read and write to the gamestate. This is
    ///intended to allow the engine to implement a wide variety of RPG mechanics - skillchecks,
    ///faction reputation gating, quest scripting, etc. This can be any function that read/writes
    ///to the gamestate as long as it produces a choice result that can be parsed. Go nuts!
    c_and_c: Option<ChecksAndConsequences>,
    ///Each entry will have at least one piece of text that displayed after that option has been chosen
    result_text: ResultText, //vec of different things that can be printed to the screen as a result, indexed by casting the result into a usize for accessing the vec of strings
}
impl IntMenuEntry {
    pub fn new(
        entry_text: String,
        vis_condition: Option<VisCondition>,
        c_and_c: Option<ChecksAndConsequences>,
        result_text: ResultText,
    ) -> Self {
        Self {
            entry_text,
            vis_condition,
            c_and_c,
            result_text,
        }
    }
}
///Datatype used for checking if an interaction menu entry should be displayed
///it should only ever need read-only access (non-mutable reference) to the gamestate
///and based on that will return a bool representing if it can be shown or not.
type VisCondition = fn(&State) -> bool;
type ChecksAndConsequences = fn(&mut State, &mut CommandBuffer) -> Option<ChoiceResult>;

#[derive(Clone, Debug, PartialEq)]
///This datatype contains the different possible results of an option in an
///interaction menu. When an entry has been chosen and checks have been run
///this is what will be referenced in order to see what will need to be printed
///but this doesn't contain any logic itself.
pub struct ResultText {
    options: Vec<String>,
}
impl ResultText {
    ///This method makes the result text for an interaction menu entry
    ///that doesn't have any result such as examining an object or
    ///pulling a lever
    pub fn new_static_result_text(result: String) -> Self {
        Self {
            options: vec![result],
        }
    }
    ///This method makes the result text for an interaction menu entry that
    ///has a binary result, such as whether or not the player has a key for a
    ///lock or has completed a requisite quest.
    pub fn new_binary_result_text(false_result: String, true_result: String) -> Self {
        Self {
            options: vec![false_result, true_result],
        }
    }
    ///This method makes the result text for an interaction menu entry that
    ///has degrees of success as a result, almost always this will be in the
    ///case of a skillcheck.
    pub fn new_deg_of_success_result_text(
        failure: String,
        partial_success: String,
        full_success: String,
    ) -> Self {
        Self {
            options: vec![failure, partial_success, full_success],
        }
    }
    ///retrieves the appropriate string for whatever result the chosen interaction menu
    ///entry has created.
    pub fn get(&self, choice: Option<ChoiceResult>) -> String {
        //testing
        if choice.is_some() {
            //actually access the contents
            match choice.unwrap() {
                ChoiceResult::BinaryResult(result) => match result {
                    false => self.options[0].clone(),
                    true => self.options[1].clone(),
                },
                ChoiceResult::DegOfSuccess(result) => match result {
                    DegreeOfSuccess::Failure => self.options[0].clone(),
                    DegreeOfSuccess::PartialSuccess => self.options[1].clone(),
                    DegreeOfSuccess::FullSuccess => self.options[2].clone(),
                },
            }
        } else {
            //if there's no result it's b/c there's only one way it'll go so just print the one result
            self.options[0].clone()
        }
    }
}
