/*
Game Data
- Constants about the game are here
- Data structures are here
- Some generation logic is here
*/

use rand::prelude::*;
use crate::game_data::BlockType::Obsidian;
use crate::game_data::ActionType::BlockCollect;

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
//resource cube generation
pub fn create_resource_cube() -> ResourceCube {
    let mut layer_1: Vec<BlockType> = vec![];
    let mut layer_2: Vec<BlockType> = vec![];
    let mut layer_3: Vec<BlockType> = vec![];
    let mut layer_4: Vec<BlockType> = vec![];
    let mut wood_left = WOOD_BLOCK_COUNT;
    let mut sand_left = SAND_BLOCK_COUNT;
    let mut stone_left = STONE_BLOCK_COUNT;
    let mut obsidian_left = OBSIDIAN_BLOCK_COUNT;
    let mut emerald_left = EMERALD_BLOCK_COUNT;

    //iterate through and fill up all the layers
    for layer in 0..3 {
        for i in 0..15 {
            //create block
            let block = gen_block(wood_left, sand_left, stone_left, obsidian_left, emerald_left);
            //decrement quotas
            match block {
                BlockType::Wood => { wood_left -= 1; }
                BlockType::Sand => { sand_left -= 1; }
                BlockType::Stone => { stone_left -= 1; }
                BlockType::Obsidian => { obsidian_left -= 1; }
                BlockType::Emerald => { emerald_left -= 1; }
            }
            //assign to layer
            match layer{
                0 => {layer_1[i] = block;}
                1 => {layer_2[i] = block;}
                2 => {layer_3[i] = block;}
                3 => {layer_4[i] = block;}
                _ => {}
            }
        }
    }
    //create our cube, and return it
    ResourceCube{
        layer_1,
        layer_2,
        layer_3,
        layer_4
    }
}
///Generate a single block,given quotas of how many blocks are left of each type.
fn gen_block(wood:i32, sand:i32, stone:i32, obsidian:i32, emerald:i32) -> BlockType{
    let mut rng = rand::thread_rng();
    let blocktype = rng.gen_range(0, 4);
    let mut block:BlockType = BlockType::Wood;//this should always be overwritten
    match blocktype{
        0 =>{
            if wood != 0 { //we have some left in our quota
                block = BlockType::Wood;
            }
            else{
                block = gen_block(wood, sand, stone, obsidian, emerald); //recusive calls are amazing.
            }
        }
        1 => {
            if sand != 0 { //we have some left in our quota
                block = BlockType::Sand;
            }
            else{
                block = gen_block(wood, sand, stone, obsidian, emerald);
            }
        }
        2 => {
            if stone != 0 { //we have some left in our quota
                block = BlockType::Stone;
            }
            else{
                block = gen_block(wood, sand, stone, obsidian, emerald);
            }
        }
        3 => {
            if obsidian != 0 { //we have some left in our quota
                block = BlockType::Obsidian;
            }
            else{
                block = gen_block(wood, sand, stone, obsidian, emerald);
            }
        }
        4 => {
            if emerald != 0 { //we have some left in our quota
                block = BlockType::Emerald;
            }
            else{
                block = gen_block(wood, sand, stone, obsidian, emerald);
            }
        }

        _ => {}
    }
    block
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

//Monster types
pub enum MobType{
    Zombie,
    Creeper,
    Enderman,
    Spider,
    Witch,
    Skeleton
}

pub struct Mob{
    mob_type:MobType,
    health:i32,
    experience:i32,
    additional_action:bool,
    keep_end_count:i32,
    keep_end_biome:BiomeType
}

//Reference Image: mobs_1

//Reference Image: mobs_2a

//Reference Image: mobs_2b

//Reference Image: mobs_3
pub const MOB_SKELETON_EXTURN:Mob = Mob{
    mob_type:MobType::Skeleton,
    health: 3,
    experience:4,
    additional_action: true,
    keep_end_count: 0,
    keep_end_biome: BiomeType::Forest
};
pub const MOB_SPIDER_EXTURN:Mob = Mob{
    mob_type: MobType::Spider,
    health: 1,
    experience: 2,
    additional_action: true,
    keep_end_count: 0,
    keep_end_biome: BiomeType::Forest
};
pub const MOB_ZOMBIE_EXTURN:Mob = Mob{
    mob_type: MobType::Zombie,
    health: 2,
    experience: 3,
    additional_action: true,
    keep_end_count: 0,
    keep_end_biome: BiomeType::Forest
};

pub enum BiomeType{
    Forest,
    Snow,
    Rocky,
    Desert
}

pub struct Buildable{
    biome:BiomeType,
    materials:Vec<BlockType>
}

pub struct Player{
    x_junction:i32,
    y_junction:i32
}

//Weapon types
pub enum WeaponType{
    Bow,
    GoldenHoe,
    WoodSword,
    StoneSword,
    IronSword,
    DiamondSword,
    StonePickaxe,
    Tnt,
    PoisonPotato
}

//Numbers of things overall
pub const RESOURCE_CUBE_COUNT:i32 = 64;
pub const TILE_COUNT:i32 = 64;
pub const WEAPON_TOKEN_COUNT:i32 = 36;
pub const WOOD_BLOCK_COUNT:i32 = 16;
pub const SAND_BLOCK_COUNT:i32 = 14;
pub const STONE_BLOCK_COUNT:i32 = 12;
pub const OBSIDIAN_BLOCK_COUNT:i32 = 10;
pub const EMERALD_BLOCK_COUNT:i32 = 12;

//Defining Building Tiles!
