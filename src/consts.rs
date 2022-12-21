/** TEXTURES */
pub const PLAYER_TEXTURE: &str = "textures/player.png";
pub const TILE_TEXTURE: &str = "textures/tile.png";
pub const WALL_TEXTURE: &str = "textures/wall.png";
pub const BOX_TEXTURE: &str = "textures/box.png";
pub const GOAL_TEXTURE: &str = "textures/goal.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/square-deal.ttf";

/** MAPS */
pub const MAP_NAMES: [&str; LEVEL_AMOUNT as usize] = [
    "maps/1.txt",
    "maps/2.txt",
    "maps/3.txt",
    "maps/4.txt",
    "maps/5.txt",
    "maps/6.txt",
    "maps/7.txt",
    "maps/8.txt",
    "maps/9.txt",
];
//for sure this can be done in a better way, I just haven't found it yet

/** DIMENSIONS */
pub const TILE_SIZE: f32 = 50.;
pub const MOVE_DELAY: f32 = 0.15;

/** Z_INDICES */
pub const BOX_Z_INDEX: f32 = 2.0;
pub const PLAYER_Z_INDEX: f32 = 2.0;
pub const WALL_Z_INDEX: f32 = 2.0;
pub const TILE_Z_INDEX: f32 = 0.0;
pub const GOAL_Z_INDEX: f32 = 1.0;

/** MISCELLANEOUS */
pub const LEVEL_AMOUNT: u32 = 9;
