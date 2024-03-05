use crate::{components::IntMenuMOI, prelude::*};
//player input function
pub fn player_input(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key;
    let control_state = state.controlstate;

    if key.is_some() {
        match control_state {
            ControlState::InInteraction => {
                interaction_input(state, commands);
            }
            ControlState::ReadingResult => {
                reading_result(state);
            }
        }
    }
}

fn interaction_input(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key;
    if key.is_some() {
        match key.unwrap() {
            VirtualKeyCode::Key1 => commands.spawn(((), IntMenuMOI { index: 0 as usize })),
            VirtualKeyCode::Key2 => commands.spawn(((), IntMenuMOI { index: 1 as usize })),
            VirtualKeyCode::Key3 => commands.spawn(((), IntMenuMOI { index: 2 as usize })),
            VirtualKeyCode::Key4 => commands.spawn(((), IntMenuMOI { index: 3 as usize })),
            VirtualKeyCode::Key5 => commands.spawn(((), IntMenuMOI { index: 4 as usize })),
            VirtualKeyCode::Key6 => commands.spawn(((), IntMenuMOI { index: 5 as usize })),
            VirtualKeyCode::Key7 => commands.spawn(((), IntMenuMOI { index: 6 as usize })),
            VirtualKeyCode::Key8 => commands.spawn(((), IntMenuMOI { index: 7 as usize })),
            VirtualKeyCode::Key9 => commands.spawn(((), IntMenuMOI { index: 8 as usize })),
            VirtualKeyCode::Key0 => commands.spawn(((), IntMenuMOI { index: 9 as usize })),
            _ => {
                //do nothing!
            }
        }
    }
}

fn reading_result(state: &mut State) {
    let key = state.key;
    if key.is_some() {
        match key.unwrap() {
            VirtualKeyCode::Escape | VirtualKeyCode::Space | VirtualKeyCode::Return => {
                state.controlstate = ControlState::InInteraction;
                //will also need to find and delete the response MOI
            }
            _ => {
                //do nothing
            }
        }
    }
}
