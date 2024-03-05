//this will hold all the components for the ECS
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

//have enum that wraps if its a boolean or a degree of success to make data components more generic

use crate::State;

///This is the base datatype for all of the engine's interaction menus.
pub struct InteractionMenu {
    options: Vec<IntMenuEntry>,
}
//need to be able to
type VisCondition = fn(&State) -> bool;
type ChecksAndConsequences = fn(&State, &mut CommandBuffer);
// type ResultText = Vec<String>;
struct ResultText {
    options: Vec<String>,
}
//need to be able to
impl ResultText {
    fn get(&self, choice: Option<ChoiceResult>) -> String {
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
enum DegreeOfSuccess {
    Failure,
    PartialSuccess,
    FullSuccess,
}
enum ChoiceResult {
    BinaryResult(bool),
    DegOfSuccess(DegreeOfSuccess),
}
//this will need to check the worldstate and then both produce a change in the world
//(probably mostly by commandbuffer and NOT direct state access)
//then there needs to be some sort of way to communicate between this and the
///This is the base datatype for every node
struct IntMenuEntry {
    entry_text: String, //this is the text displayed on the root level of the entry menu
    vis_condition: Option<VisCondition>, //lets each entry have the option of being hidden unless a condition is met
    c_and_c: Option<ChecksAndConsequences>, //function that looks at the world and edits
    result: Option<ChoiceResult>,        //result of the choice
    result_text: ResultText, //vec of different things that can be printed to the screen as a result, indexed by casting the result into a usize for accessing the vec of strings
}
