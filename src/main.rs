mod prelude {
    pub use crate::components::*;
    pub use crate::control_state::*;
    pub use crate::dialogue::*;
    pub use crate::interactionmenus::*;
    pub use crate::intmenu_library::*;
    pub use crate::quests::*;
    pub use crate::State;
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
    pub use std::collections::HashMap;
}
mod components;
mod control_state;
mod dialogue;
mod init;
mod interactionmenus;
mod intmenu_library;
mod quests;
mod systems;

///The gamestate
pub struct State {
    ecs: World,
    key: Option<VirtualKeyCode>,
    controlstate: ControlState,
    int_menu_db: InteractionMenuDatabase,
    player: Entity,
    quest_db: QuestDatabase,
}
impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let log: Vec<String> = Vec::new();
        init_ecs(&mut ecs);
        let player = ecs.query::<&Player>().iter().nth(0).unwrap().0;
        Self {
            ecs,
            key: None,
            controlstate: ControlState::InInteraction,
            int_menu_db: crate::init::init_int_menu_db(),
            player,
            quest_db: crate::init::init_quest_db(),
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        //tick function goes here
        ctx.set_active_console(0);
        ctx.cls();
        self.key = ctx.key;
        systems::run_systems(self);
        render_draw_buffer(ctx).expect("Render error")
    }
}
use init::init_ecs;
use prelude::*;
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dialogue Skunkworks")
        .build()?;
    main_loop(context, State::new())
}
