use ggez;
use ggez::event;
use ggez::graphics;
use ggez::filesystem;
use ggez::nalgebra as na;

use std::env;
use std::path;

mod map;
pub use map::*;

mod template_map;
pub use template_map::*;




struct MainState {
    pos_x: f32,
    map: map::Map,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0, map: map::test_map_from_template() };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        map::draw_map(&self.map, ctx)
    }
}

pub fn main() -> ggez::GameResult { 
    let mut cb = ggez::ContextBuilder::new("Sokoban", "BittersweetPuff");
    let ws = ggez::conf::WindowSetup {
        title: "Sokoban".to_owned(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };
    cb = cb.window_setup(ws);
   
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }



    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}