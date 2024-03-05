use crate::prelude::*;
pub fn draw_screen(state: &mut State) {
    match &state.controlstate {
        ControlState::SelectingInteraction => draw_interaction_menu(state),
        ControlState::ReadingResult => draw_result(state),
    }
}

fn draw_interaction_menu(state: &mut State) {
    let draw_batch = DrawBatch::new();
}

fn draw_result(state: &mut State) {
    let draw_batch = DrawBatch::new();
}
//this is the function that will draw the menu
