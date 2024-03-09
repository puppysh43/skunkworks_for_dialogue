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
    let active_interactionmenu = get_active_interactionmenu(state).unwrap();
    let visible_entries = active_interactionmenu.get_visible_entries(state);
    let mut valid_entries: Vec<String> = Vec::new();
    for index in visible_entries.iter() {
        //get entry text of option and add it to the vec of valid entries
        valid_entries.push(active_interactionmenu.get_entry(*index).get_entry_text());
    }
    let mut line_num = 0;
    //need to print the header of the interaction menu
    let formatted_header = greedy_word_wrap(active_interactionmenu.get_header_text(), 60);
    for line in formatted_header {
        draw_batch.print(Point::new(10, line_num), line);
        line_num += 1;
    }
    line_num += 2;
    //then print all the visible options
    let mut option_num = 1;
    for line in valid_entries {
        let fmt_option = format!("{}. {}", option_num, line);
        draw_batch.print(Point::new(2, line_num), fmt_option);
        line_num += 2;
        option_num += 1;
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
