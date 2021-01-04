//Builders and Biomes reimplementation
//Minecraft and Minecraft: Builders and biomes are trademarks of Mojang and Microsoft.


//module declarations
mod game_data;
mod net;
//using statments
use macroquad::prelude::*;


//====Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: format!("Builders and Biomes").to_owned(),
        window_width: 1280,
        window_height: 960,
        ..Default::default()
    }
}

//====Main
#[macroquad::main(window_conf)]
async fn main() {
    //===Setup
    let mut scoring_rounds_left = 3;





    //===Gameloop
}
