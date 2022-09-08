mod hashing;
mod ppacket;
mod client;
mod jsonize;
mod connections;
use ppacket::PPacket;
use tokio::net::{TcpListener, TcpStream};
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::{env, time::Duration};
use colored::Colorize;
use crate::logger::Logger;
mod logger;
use logger::LOGTYPE;
#[allow(dead_code, unused_imports)]

fn get_hardcoded_servers() -> Vec<&'static str>{
    let hardcoded_addresses : Vec<&str> = vec!["127.0.0.1:1234" , "127.0.0.1:3214"];
    hardcoded_addresses
}

const APPLICATION_PORT : u16 = 48134;
const PORT_NUMBER : u16 = 1234;



#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    connections::clean_server().await;
    let args: Vec<String> = env::args().collect();
    

    let mode = if args.len()>1 && args[1] == "hardcode" { "hardcode" } else { "client" }; 
    format!("mode : {}",mode.bright_white()).log(LOGTYPE::DEBUG);
    tokio::spawn(async move{
        server_handle(mode).await;
    });
    tokio::spawn(async move{
        application_handle().await;
    });
    tokio::spawn(async move {hashing::hash_remover().await;});
    
    std::thread::sleep(std::time::Duration::from_millis(250));    
    let mut stream : TcpStream = TcpStream::connect("0.0.0.0:1234").await.unwrap();
    let packet : PPacket = PPacket::from_str(1 , format!("{{\"ip\":\"127.0.0.1\" , \"port\":\"{}\"}}" , 1234).as_str());
    //format!("overall checksum : {}", packet.overall_checksum()).log(LOGTYPE::DEBUG);
    if client::send_ppacket(&mut stream, &packet).await.is_err(){
        format!("failed to send packet : {}", packet.overall_checksum()).log(LOGTYPE::ERROR);
    }
    format!("Sent {}.." , &packet.overall_checksum()[0..16]).as_str().log(LOGTYPE::INFO);

    for _i in 100..110{
        std::thread::sleep(std::time::Duration::from_millis(250));    
        let mut stream : TcpStream = TcpStream::connect("0.0.0.0:1234").await.unwrap();
        let packet : PPacket = PPacket::from_str(1 , format!("{{\"ip\":\"192.168.1.1\" , \"port\":\"{}\"}}" , _i).as_str());
        //format!("overall checksum : {}", packet.overall_checksum()).log(LOGTYPE::DEBUG);
        if client::send_ppacket(&mut stream, &packet).await.is_err(){
            format!("failed to send packet : {}", packet.overall_checksum()).log(LOGTYPE::ERROR);
        }
        format!("Sent {}.." , &packet.overall_checksum()[0..16]).as_str().log(LOGTYPE::INFO);
    }
    loop{
        std::thread::sleep(Duration::from_secs(4));
    }
    
}


async fn server_handle(mode :  &'static str) {
    let address : String = format!("{}:{}" , "0.0.0.0" , PORT_NUMBER);
        if let Ok(listener) = TcpListener::bind(address).await{ 
            "Listening on Server side".bright_red().to_string().log(LOGTYPE::INFO);
            loop{
                let (mut stream , _address) = listener.accept().await.unwrap();
                format!("New connection from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO);
                tokio::spawn(async move{
                client::handle_client(&mut stream , mode).await;
                });
            }
        }
        else{
            "Failed to bind server, check port availability".log(LOGTYPE::ERROR); 
        }    
}

async fn application_handle(){
    let address : String = format!("{}:{}" , "127.0.0.1" , APPLICATION_PORT);
        if let Ok(listener) = TcpListener::bind(address).await{ 
            "Listening on application side".bright_red().to_string().log(LOGTYPE::INFO);
            loop{
                let (mut stream , _address) = listener.accept().await.unwrap();
                format!("New connection from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO);
                tokio::spawn(async move {
                    client::handle_application(&mut stream).await;
                });
            }
        }
        else{
            "Failed to bind app communicator, check port availability".log(LOGTYPE::ERROR); 
        }    
}