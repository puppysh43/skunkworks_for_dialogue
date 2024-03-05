mod prelude {
    pub use crate::State;
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
}
mod components;
mod control_state;
mod systems;

pub struct State {
    ecs: World,
    key: Option<VirtualKeyCode>,
    //controlstate:
    // player: Entity,
    log: Vec<String>,
}
impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let log: Vec<String> = Vec::new();
        Self {
            ecs,
            key: None,
            log,
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
use prelude::*;
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dialogue Skunkworks")
        .build()?;
    main_loop(context, State::new())
}
