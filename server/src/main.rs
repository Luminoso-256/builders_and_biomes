//Builders and Biomes reimplementation
//Minecraft and Minecraft: Builders and biomes are trademarks of Mojang and Microsoft.


use std::net::{TcpListener, TcpStream};
use std::io::Read;

//module declarations
mod game_data;
mod packet;

//====Main
fn main() {
    //===Setup
    let mut scoring_rounds_left = 3;


    //===Initializing server
    println!("Binding to socket 24454");
    let listener = TcpListener::bind("127.0.0.1:24454").unwrap();

    //listen for incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection recieved");
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}