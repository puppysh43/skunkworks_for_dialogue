use crate::prelude::*;
//this will have all of the functions that generate the interaction menus

pub fn init_quest_db() -> QuestDatabase {
    let mut quest_database: HashMap<String, Quest> = HashMap::new();
    let boulder_quest = Quest::new(vec![
        (
            "PushedBoulder".to_string(),
            Quest::new(vec![
                ("Failure".to_string(), Quest::new(Vec::new())),
                ("PartialSuccess".to_string(), Quest::new(Vec::new())),
                ("FullSuccess".to_string(), Quest::new(Vec::new())),
            ]),
        ),
        ("ExplodedBoulder".to_string(), Quest::new(Vec::new())),
    ]);
    quest_database.insert("Boulder".to_string(), boulder_quest);
    // let

    QuestDatabase::new(quest_database)
}

pub fn init_int_menu_db() -> InteractionMenuDatabase {
    let mut int_menu_db_contents: HashMap<String, InteractionMenu> = HashMap::new();
    let mut boulder = InteractionMenu::new_blank("A large boulder lies in front of you, blocking the tunnel you need to pass through to reach your goal. It looks immensely heavy, a craggy face of granite only the strongest adventurers could move.".to_string());
    let examine_boulder = IntMenuChoice::new(
        "Examine The Boulder".to_string(),
        None,
        None,
        ResultText::new_static_result_text("Despite its invincible exterior, the boulder is spiderwebbed with countless hairline fractures suggesting it could easily be broken apart with explosives or digging equipment.".to_string())
    );
    boulder.add_entry(examine_boulder);
    let move_boulder = IntMenuChoice::new("[Athletics Check] Try and push the boulder aside.".to_string(), None,
    Some(move_boulder_skillcheck), ResultText::new_deg_of_success_result_text("You fail to move the boulder at all, and are left only sweaty and out of breath for your effort".to_string(),
            "You're able to rock the boulder an an axis just enough to rotate it in the hallway before it gets stuck again, shifting it enough to leave a small crack. You can probably fit, but it might hurt.".to_string() , 
        "In a feat of herculean strength you're able to completely push the boulder away from the entrance of the tunnel, clearing your way.".to_string()));
    boulder.add_entry(move_boulder);
    let squeeze_past = IntMenuChoice::new("[Acrobatics Check] Try and squeeze through the gap between the boulder and the wall.".to_string(), Some(has_boulder_part_moved), Some(

        
    ), ResultText::new_deg_of_success_result_text(String::from("You try to squeeze through the gap between the boulder and the wall but can't quite make it, hurting yourself in the process."), String::from("You at first slip through with ease, but part of you catches on the rough rock and you find yourself stuck. Panic setting in you're able to wriggle free, realizing only after the adrenaline wears off that a sharp outcrop of stone has gouged your leg. [-1 HP]"), String::from("With a single fluid movement you're able to effortlessly squeeze through the gap.")) );
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
            state
                .quest_db
                .quest("Boulder")
                .step("PushedBoulder")
                .unwrap()
                .step("PartialSuccess")
                .unwrap()
                .flag_complete();
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::PartialSuccess));
        }
        RollResult::FullSuccess => {
            //in the real game this would go into the state and delete the position component
            //of the boulder so that it would dissapear from the map and no longer block collissions
            state
                .quest_db
                .quest("Boulder")
                .step("PushedBoulder")
                .unwrap()
                .step("FullSuccess")
                .unwrap()
                .flag_complete();
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::FullSuccess));
        }
    }
}
fn squeeze_boulder_skillcheck(state: &mut State, commands: &mut CommandBuffer) -> Option<ChoiceResult>{
    let player_skills = get_player_skills(state);
    //moving a boulder is a big task, have it a -1 athletics skillcheck
    let check_result = player_skills.skillcheck(SkillType::Reflexes, 0);
    match check_result {
        RollResult::Failure => {
            //remove 1 HP and do NOT move the player
                        return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::Failure));
        }
        RollResult::PartialSuccess => {
            //in the full game you would move the player, maybe make it so they can't move again
            //and deal 1 HP damage.
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::PartialSuccess));
        }
        RollResult::FullSuccess => {
            //in the actual game move the player and maybe remove the solid tag from the entity
            return Some(ChoiceResult::DegOfSuccess(DegreeOfSuccess::FullSuccess));
        }
    }
}

fn has_boulder_part_moved(state: &State) -> bool {
    state
        .quest_db
        .quest("Boulder")
        .step("PushedBoulder")
        .unwrap()
        .step("PartialSuccess")
        .unwrap()
        .status()
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
