//this will hold the controlstate enum used for potentially navigating menus although
//an approach without the need for a controlstate would be nice!
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlState {
    // SelectingInteraction,//picking out an interaction from the main menu
    InInteraction, //actively in an interaction
    ReadingResult, //reading the result of an interaction
}
