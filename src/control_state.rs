//this will hold the controlstate enum used for potentially navigating menus although
//an approach without the need for a controlstate would be nice!
pub enum ControlState {
    SelectingInteraction,
    InInteraction,
    ReadingResult,
}
