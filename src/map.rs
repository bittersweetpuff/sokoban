use ggez::graphics;
use ggez::nalgebra as na;

use std::path;

use super::template_map;


pub const MAPWIDTH: usize = 16;
pub const MAPHEIGHT: usize = 16;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;
pub const TILESIZE: i32 = 32;

#[derive(Clone, Copy)]
pub enum TileType {
    Wall,
    Player, 
    Target,
    Chest,
    Floor,
}



pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_border_walls(&mut self) {
        let mut y = 0;
        let mut x = 0;
        for y in 0..= self.height - 1  {
            for x in 0..= self.width - 1 {
                if (x == 0 || x == self.width -1) || (y == 0 || y == self.height - 1){
                    let idx = self.xy_idx(x, y);
                    self.tiles[idx] = TileType::Wall;
                }
            }
        }
    }

    fn apply_player(&mut self) {
        let idx = self.xy_idx(self.width/2, self.height/2);
        self.tiles[idx] = TileType::Player;
    }
}

pub fn new_test_map() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Floor; MAPCOUNT],
        width: MAPWIDTH as i32,
        height: MAPHEIGHT as i32,
    };
    map.apply_border_walls();
    map.apply_player();
    map
}

pub fn map_from_template(template: template_map::TemplateMap) -> Map {
    let mapcount = template.width * template.height;
    let mut map = Map {
        tiles: vec![TileType::Floor; mapcount as usize],
        height: template.height,
        width: template.width
    };

    for (idx, glyph) in template.tiles.chars().enumerate() {
        let tile: TileType;
        match glyph {
            '.' => {
                tile = TileType::Floor;
            },
            '#' => {
                tile = TileType::Wall;
            }
            'x' => {
                tile = TileType::Target;
            }
            '@' => {
                tile = TileType::Player;
            }
            'o' => {
                tile = TileType::Chest;
            }
            _ => {
                tile = TileType::Floor;
            }
        }
        map.tiles[idx] = tile;
    }
    
    map
}

pub fn test_map_from_template() -> Map {
    let template = template_map::TemplateMap {
        height: 9,
        width: 9,
        tiles: String::from("##########.......##.x...x.##.......##.o...o.##.......##.......##...@...##########")
    };

    map_from_template(template)
}

pub fn draw_map(map: &Map, ctx: &mut ggez::Context) -> ggez::GameResult {
    let floor = graphics::Image::new(ctx, path::Path::new("/floor.png"))?;
    let wall = graphics::Image::new(ctx, path::Path::new("/wall.png"))?;
    let player = graphics::Image::new(ctx, path::Path::new("/player.png"))?;
    let target = graphics::Image::new(ctx, path::Path::new("/target.png"))?;
    let chest = graphics::Image::new(ctx, path::Path::new("/chest.png"))?;
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
    let mut y = 0.0;
    let mut x = 0.0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        match tile {
            TileType::Floor => {
                graphics::draw(ctx, &floor, (na::Point2::new(x*(TILESIZE as f32), y*(TILESIZE as f32)),));
            }
            TileType::Wall => {
                graphics::draw(ctx, &wall, (na::Point2::new(x*(TILESIZE as f32), y*(TILESIZE as f32)),));
            }
            TileType::Player => {
                graphics::draw(ctx, &player, (na::Point2::new(x*(TILESIZE as f32), y*(TILESIZE as f32)),));
            }
            TileType::Chest => {
                graphics::draw(ctx, &chest, (na::Point2::new(x*(TILESIZE as f32), y*(TILESIZE as f32)),));
            }
            TileType::Target => {
                graphics::draw(ctx, &target, (na::Point2::new(x*(TILESIZE as f32), y*(TILESIZE as f32)),));
            }
        }
        x += 1.0;
        if x > (map.width as f32) - 1.0 {
            x = 0.0;
            y += 1.0;
        }
    }

    graphics::present(ctx)?;
    Ok(())

}
