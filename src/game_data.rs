/*
Game Data
- Constants about the game are here
- Data structures are here
- Some generation logic is here
*/

//main game state struct
pub struct GameState {

}

//Tile type
pub enum TileType{
    Building,
    Monster,
    Empty
}

//Player Colors
pub enum PlayerColor{
    Red,
    Blue,
    Yellow,
    Green
}

//actions
pub enum ActionType{
    BlockCollect,
    Explore,
    Build,
    Fight,
    WeaponCollect
}
//Resource cube

//layer 1 is the bottom
//layer 4 is the top
//each layer is a vec<BlockType> mapped out like this:
//0, 1, 2, 3
//4, 5, 6, 7
//8, 9, 10, 11
//12, 13, 14, 15
pub struct ResourceCube{
    layer_1:Vec<BlockType>,
    layer_2:Vec<BlockType>,
    layer_3:Vec<BlockType>,
    layer_4:Vec<BlockType>
}

//Main board - 16 structs of 4 each
pub struct GameBoard{
    tile_0_0:Vec<TileType>,
    tile_1_0:Vec<TileType>,
    tile_2_0:Vec<TileType>,
    tile_3_0:Vec<TileType>,
    tile_0_1:Vec<TileType>,
    tile_1_1:Vec<TileType>,
    tile_2_1:Vec<TileType>,
    tile_3_1:Vec<TileType>,
    tile_0_2:Vec<TileType>,
    tile_1_2:Vec<TileType>,
    tile_2_2:Vec<TileType>,
    tile_3_2:Vec<TileType>,
    tile_0_3:Vec<TileType>,
    tile_1_3:Vec<TileType>,
    tile_2_3:Vec<TileType>,
    tile_3_3:Vec<TileType>,
}



//block types
pub enum BlockType{
    Wood,
    Sand,
    Stone,
    Obsidian,
    Emerald
}

//Numbers of things
pub const RESOURCE_CUBE_COUNT:i32 = 64;
pub const TILE_COUNT:i32 = 64;
pub const WEAPON_TOKEN_COUNT:i32 = 36;
pub const WOOD_BLOCK_COUNT:i32 = 16;
pub const SAND_BLOCK_COUNT:i32 = 14;
pub const STONE_BLOCK_COUNT:i32 = 12;
pub const OBSIDIAN_BLOCK_COUNT:i32 = 10;
pub const EMERALD_BLOCK_COUNT:i32 = 12;
