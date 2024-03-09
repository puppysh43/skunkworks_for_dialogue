use crate::prelude::*;
//this will have all of the functions that generate the interaction menus
pub fn init_int_menu_db() -> InteractionMenuDatabase {
    let mut int_menu_db_contents: HashMap<String, InteractionMenu> = HashMap::new();
    let mut boulder = InteractionMenu::new_blank("A large boulder lies in front of you, blocking the tunnel you need to pass through to reach your goal. It looks immensely heavy, a craggy face of granite only the strongest adventurers could move.".to_string());
    let examine_boulder = IntMenuEntry::new(
        "Examine The Boulder".to_string(),
        None,
        None,
        ResultText::new_static_result_text("Despite its invincible exterior, the boulder is spiderwebbed with countless hairline fractures suggesting it could easily be broken apart with explosives or digging equipment.".to_string())
    );
    boulder.add_entry(examine_boulder);

    int_menu_db_contents.insert("Boulder".to_string(), boulder);
    InteractionMenuDatabase::new(int_menu_db_contents)
}

pub fn init_ecs(ecs: &mut World) {
    ecs.spawn((
        (),
        InteractionMenuKey("Boulder".to_string()),
        ActiveInteractionMenu,
    ));
}
