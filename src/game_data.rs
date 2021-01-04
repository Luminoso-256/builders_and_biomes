/*
Game Data
- Constants about the game are here
- Data structures are here
- no logic is here. that goes in main
*/



//main game state struct
pub struct GameState {

}

//Tile type
pub enum TileType{
    Building,
    Monster
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



//Numbers of things
pub const RESOURCE_CUBE_COUNT:i32 = 64;
pub const TILE_COUNT:i32 = 64;
pub const WEAPON_TOKEN_COUNT:i32 = 36;

