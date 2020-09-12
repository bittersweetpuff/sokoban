use ggez;
use ggez::event;
use ggez::graphics;
use ggez::filesystem;
use ggez::nalgebra as na;

use std::env;
use std::io;
use std::path;


const tile_x:i32 = 32;
const tile_y:i32 = 32;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;

        let wall = graphics::Image::new(ctx, path::Path::new("/player.png"))?;
        graphics::draw(ctx, &wall, (na::Point2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez");
   
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }



    let (ctx, event_loop) = &mut cb.build()?;
    println!("Full filesystem info: {:#?}", ctx.filesystem);


    let dir_contents: Vec<_> = filesystem::read_dir(ctx, "/")?.collect();
    println!("Directory has {} things in it:", dir_contents.len());
    for itm in dir_contents {
        println!("   {:?}", itm);
    }
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}