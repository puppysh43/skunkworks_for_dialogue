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
    let move_boulder = IntMenuEntry::new("[Athletics Check] Try and push the boulder aside.".to_string(), None,
    Some(move_boulder_skillcheck), ResultText::new_deg_of_success_result_text("You fail to move the boulder at all, and are left only sweaty and out of breath for your effort".to_string(),
            "You're able to rock the boulder an an axis just enough to rotate it in the hallway, shifting it enough to leave a small crack. You can probably fit, but it might hurt.".to_string() , 
        "In a feat of herculean strength you're able to completely push the boulder away from the entrance of the tunnel, clearing your way.".to_string()));
    boulder.add_entry(move_boulder);
    int_menu_db_contents.insert("Boulder".to_string(), boulder);
    InteractionMenuDatabase::new(int_menu_db_contents)
}

pub fn init_ecs(ecs: &mut World) {
    ecs.spawn((
        (),
        InteractionMenuKey("Boulder".to_string()),
        ActiveInteractionMenu,
    ));
    ecs.spawn((
        (),
        Player,
        Skills::new(0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    ));
}

fn move_boulder_skillcheck(
    state: &mut State,
    commands: &mut CommandBuffer,
) -> Option<ChoiceResult> {
    let player_skills = get_player_skills(state);
    //moving a boulder is a big task, have it a -1 athletics skillcheck
    let check_result = player_skills.skillcheck(SkillType::Athletics, -1);
    match check_result {
        RollResult::Failure => {
            //if this were the real game we would subtract some fatigue points or something from the player
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::Failure));
        }
        RollResult::PartialSuccess => {
            //in a real game this would switch a flag in the state that was hiding another
            //option in the menu to let the player squeeze through in exchange for taking
            //damage if they fail a reflexes/dexterity check
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::PartialSuccess));
        }
        RollResult::FullSuccess => {
            //in the real game this would go into the state and delete the position component
            //of the boulder so that it would dissapear from the map and no longer block collissions
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::FullSuccess));
        }
    }
}

fn get_player_skills(state: &State) -> Skills {
    let player = state.player.clone();
    let player_skills = state
        .ecs
        .query_one::<&Skills>(player)
        .expect("Player doesn't have a skills component.")
        .get()
        .unwrap()
        .clone();
    player_skills
}
