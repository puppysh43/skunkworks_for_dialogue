use crate::prelude::*;
pub fn draw_screen(state: &mut State) {
    match &state.controlstate {
        // ControlState::SelectingInteraction => draw_interaction_select()
        ControlState::InInteraction => draw_interaction_menu(state),
        ControlState::ReadingResult => draw_result(state),
    }
}

fn draw_interaction_menu(state: &mut State) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    //will need to go through and check to see which ones are visible
    let mut int_menu_query = state.ecs.query::<&InteractionMenu>();
    let valid_entries: Vec<String> = Vec::new();
    for (_, int_menu) in int_menu_query.iter() {
        
    }
    draw_batch.submit(5000).expect("Batch Error");
}

fn draw_result(state: &mut State) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    //to draw the result we need to query the result MOI and then feed it into the active interaction menu entry
    //to get the result text that needs to be displayed on screen
    draw_batch.submit(5000).expect("Batch Error");
}
//this is the function that will draw the menu

fn greedy_word_wrap(raw: String, width: i32) -> Vec<String> {
    let raw_split = raw.as_str().split_ascii_whitespace();
    let mut fmt_string: Vec<String> = Vec::new();
    let mut temp_string = String::new();
    for word in raw_split {
        if (temp_string.len() as i32 + word.len() as i32) <= width {
            temp_string.push(' ');
            temp_string = temp_string + word;
        } else {
            fmt_string.push(temp_string.clone());
            temp_string.clear();
            temp_string = temp_string + word;
        }
    }
    fmt_string.push(temp_string);
    fmt_string
}
