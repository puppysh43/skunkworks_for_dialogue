use crate::prelude::*;
mod interaction;
mod player_input;
mod render_menu;
//this will hold all the systems run every tick!
pub fn run_systems(state: &mut State) {
    let commands = &mut CommandBuffer::new();
    player_input::player_input(state, commands);

    render_menu::draw_screen(state);
    //this is where you would run all the individual systems
    //get player input function
    //process interaction menu MOI if present
    //print whatever you need to on the screen based on control state
}

/*
need to be able to run see what conversation the player is tagged as being in
decide what options of the menu to show
and take input (menu will simply be labeled a b c d etc)
and then based on what is selected run the needed calculations let the person read the results before
pressing any key to continue back to the original menu

player input system will check for keypresses, see if said keypress can map to a valid index in the interaction menu
and if so will send a message of intent to select that option on the interaction menu
*/
