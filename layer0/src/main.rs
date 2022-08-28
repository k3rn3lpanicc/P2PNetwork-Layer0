mod hashing;
mod ppacket;
mod client;
mod jsonize;
use ppacket::PPacket;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::fs;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::env;
use colored::Colorize;

use crate::logger::Logger;
mod logger;

#[allow(dead_code, unused_imports)]

fn get_hardcoded_servers() -> Vec<&'static str>{
    let hardcoded_addresses : Vec<&str> = vec!["127.0.0.1:1234" , "127.0.0.1:3214"];
    hardcoded_addresses
}

const APPLICATION_PORT : u16 = 48134;
const PORT_NUMBER : u16 = 1234;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = if args.len()>1 && args[1] == "hardcode" { "hardcode" } else { "client" }; 
    println!("mode : {}",mode);
    tokio::spawn(async move{
        server_handle(&mut mode).await;
    });
    tokio::spawn(async move{
        application_handle().await;
    });
    tokio::spawn(async move {hashing::hash_remover().await;});


    let packet : PPacket = PPacket::from_str(1 , r##"{"ip":"192.168.1.1" , "port":"1234"}"##);
    println!("overall checksum : {}", packet.overall_checksum());
    let mut stream : TcpStream = TcpStream::connect("0.0.0.0:1234").await.unwrap();
    //tokio::time::sleep(Duration::from_secs(3)).await;
    for i in 0..100{
    std::thread::sleep(std::time::Duration::from_millis(1000));
    client::send_ppacket(&mut stream, &packet).await;
    format!("Sent {}" , packet.overall_checksum().bright_magenta()).as_str().log(logger::LOGTYPE::INFO);
    //println!("Sent {}" , packet.overall_checksum());
    }
    loop{}
}


async fn server_handle(mode :  &'static str) {
    let address : String = format!("{}:{}" , "0.0.0.0" , PORT_NUMBER);
        if let Ok(listener) = TcpListener::bind(address).await{ 
            println!("Listening on Server side");
            loop{
                let (mut stream , _address) = listener.accept().await.unwrap();
                println!("New connection from {}", _address);
                tokio::spawn(async move{
                    client::handle_client(&mut stream , mode).await;
                });
                println!("Handed the connection to a client handler");
            }
        }
        else{
            println!("Failed to bind server, check port availability"); 
        }    
}

async fn application_handle(){
    let address : String = format!("{}:{}" , "127.0.0.1" , APPLICATION_PORT);
        if let Ok(listener) = TcpListener::bind(address).await{ 
            println!("Listening on application side");
            loop{
                let (mut stream , _address) = listener.accept().await.unwrap();
                println!("New connection from {}", _address);
                tokio::spawn(async move {
                    client::handle_application(&mut stream).await;
                });
                println!("Handed the application to a aplication handler");
            }
        }
        else{
            println!("Failed to bind app communicator, check port availability"); 
        }    
}