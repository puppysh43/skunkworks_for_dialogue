use crate::prelude::*;
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
    //get the list of visible entries and index that instead of indexing the interaction menu
    //directly to make sure the right thing is always selected regardless of what the player can see.
    let active_interactionmenu = get_active_interactionmenu(state).unwrap();
    let visible_entries = active_interactionmenu.get_visible_entries(state);
    let key = state.key;
    if key.is_some() {
        match key.unwrap() {
            VirtualKeyCode::Key1 => {
                if visible_entries.len() >= 1 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[0],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key2 => {
                //make sure the play even has this many options before sending an MOI!
                if visible_entries.len() >= 2 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[1],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key3 => {
                if visible_entries.len() >= 3 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[2],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key4 => {
                if visible_entries.len() >= 4 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[3],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key5 => {
                if visible_entries.len() >= 5 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[4],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key6 => {
                if visible_entries.len() >= 6 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[5],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key7 => {
                if visible_entries.len() >= 7 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[6],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key8 => {
                if visible_entries.len() >= 8 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[7],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key9 => {
                if visible_entries.len() >= 9 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[8],
                        },
                    ));
                }
            }
            VirtualKeyCode::Key0 => {
                if visible_entries.len() >= 10 {
                    commands.spawn((
                        (),
                        IntMenuMOI {
                            index: visible_entries[9],
                        },
                    ));
                }
            }
            VirtualKeyCode::Escape => {
                //in the main game this will exit the active interaction by removing the active tag component
                //from whatever entity has an interactionmenu ID
            }
            _ => {
                //do nothing!
            }
        }
    }
}

fn reading_result(state: &mut State) {
    let mut commands = CommandBuffer::new();
    let key = state.key;
    if key.is_some() {
        match key.unwrap() {
            VirtualKeyCode::Escape | VirtualKeyCode::Space | VirtualKeyCode::Return => {
                state.controlstate = ControlState::InInteraction;
                for (moi_id, _) in state.ecs.query::<&IntMenuResult>().iter() {
                    commands.despawn(moi_id);
                }
                commands.run_on(&mut state.ecs);
                //will also need to find and delete the response MOI
            }
            _ => {
                //do nothing
            }
        }
    }
}
