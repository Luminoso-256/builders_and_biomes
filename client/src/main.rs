//Builders and Biomes reimplementation
//Minecraft and Minecraft: Builders and biomes are trademarks of Mojang and Microsoft.


//module declarations
mod game_data;
mod packet;
mod client_data;

//using statments
use macroquad::prelude::*;
use std::net::{TcpStream};
use std::io::{Read, Write};

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
//#[macroquad::main(window_conf)]
async fn main() {
    //===Setup

}
