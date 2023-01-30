/** TEXTURES */
pub const PLAYER_TEXTURE: &str = "textures/player.png";
pub const TILE_TEXTURE: &str = "textures/tile.png";
pub const WALL_TEXTURE: &str = "textures/wall.png";
pub const BOX_TEXTURE: &str = "textures/box.png";
pub const GOAL_TEXTURE: &str = "textures/goal.png";
pub const BOX_ON_GOAL_TEXTURE: &str = "textures/box_on_goal.png";
pub const ICE_TEXTURE: &str = "textures/ice.png";
pub const WARP_TEXTURE: &str = "textures/warp.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/square-deal.ttf";
pub const LEVEL_FONT: &str = "fonts/pixel NES.otf";

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
pub const IMAGE_SIZE: f32 = 16.;
pub const MOVE_ANIMATION_TIME: f32 = 0.2;

/** Z_INDICES */
pub const OBJECT_Z_INDEX: f32 = 2.0;
pub const FLOOR_Z_INDEX: f32 = 1.0;

/** MISCELLANEOUS */
pub const LEVEL_AMOUNT: usize = 9;
pub const MAX_MAPS: usize = 10;
pub const INITIAL_MAP: usize = 0;
