//this will hold the controlstate enum used for potentially navigating menus although
//an approach without the need for a controlstate would be nice!
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlState {
    SelectingInteraction,
    // InInteraction,
    ReadingResult,
}
