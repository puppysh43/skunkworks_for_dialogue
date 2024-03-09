use crate::prelude::*;
//this module will hold the various commonly used helper functions.
pub fn get_active_interactionmenu(state: &State) -> Option<InteractionMenu> {
    let mut interaction_menu_key = String::new(); //make a blank string to hold the key after you query for it
    let mut int_menu_query = state
        .ecs
        .query::<With<&InteractionMenuKey, &ActiveInteractionMenu>>();
    //query the key of the interaction menu
    for (_, int_menu_key) in int_menu_query.iter() {
        interaction_menu_key = int_menu_key.0.clone();
    }
    //using the active key you got from the ecs retrieve a copy of the active interaction menu from the database
    //in the state
    let interaction_menu = state.int_menu_db.get_interaction_menu(interaction_menu_key);
    interaction_menu
}
