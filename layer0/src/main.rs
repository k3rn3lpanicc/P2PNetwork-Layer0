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

pub fn get_hardcoded_servers() -> Vec<&'static str>{
    let hardcoded_addresses : Vec<&str> = vec!["127.0.0.1:1234"];
    hardcoded_addresses
}

const APPLICATION_PORT : u16 = 48134;
const PORT_NUMBER : i64 = 1234;



#[tokio::main]
async fn main() {
    connections::APP_NUMBER.to_string().log(LOGTYPE::DEBUG);
    env::set_var("RUST_BACKTRACE", "1");
    connections::create_database();
    //connections::clean_server().await;
    let args: Vec<String> = env::args().collect();
    // 1: cargo run 2>/dev/null port hardcode 2: cargo run 2>/dev/null port

    let mode = if args.len()>2 && args[2] == "hardcode" { "hardcode" } else { "client" };
    let port_number_var : i64 = if args.len()>1 { args[1].parse::<i64>().unwrap() } else { PORT_NUMBER }; 
    
    format!("mode : {}",mode.bright_white()).log(LOGTYPE::DEBUG);
    tokio::spawn(async move{
        server_handle(mode,port_number_var as u16).await;
    });
    tokio::spawn(async move{
        application_handle().await;
    });
    tokio::spawn(async move{hashing::hash_remover().await;});

    for _i in 101..111{
        std::thread::sleep(std::time::Duration::from_millis(250));    
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1234").await{
            let packet : PPacket = PPacket::from_str(1 , format!("{{\"ip\":\"192.168.1.7\" , \"port\":\"{}\"}}" , _i).as_str());
            //format!("overall checksum : {}", packet.overall_checksum()).log(LOGTYPE::DEBUG);
            if client::send_ppacket(&mut stream, &packet).await.is_err(){
                format!("failed to send packet : {}", packet.overall_checksum()).log(LOGTYPE::ERROR);
            }
            else{
                format!("Sent {}.." , &packet.overall_checksum()[0..16]).as_str().log(LOGTYPE::INFO);
            }
        }
        else{
            "Couldn't connect to server".log(LOGTYPE::ERROR);
        }
    }
    loop{
        std::thread::sleep(Duration::from_secs(10));
    }
    
}


async fn server_handle(mode :  &'static str , port_number : u16) {
    let address : String = format!("{}:{}" , "0.0.0.0" , port_number);
    if let Ok(listener) = TcpListener::bind(address).await{ 
        format!("Listening on Server side : {}:{}" , "0.0.0.0".bright_red() , port_number.to_string().bright_red()).log(LOGTYPE::INFO);
        loop{
            "Waiting for connection..".bright_red().to_string().log(LOGTYPE::INFO);
            match listener.accept().await{
                Ok((mut stream , _address)) => {
                    format!("New connection from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO);
                    tokio::spawn(async move{
                    client::handle_client(&mut stream , mode).await;
                    });
                },
                Err(err)=>{
                    format!("Error while accepting connection : {}", err.to_string().bright_red()).log(LOGTYPE::ERROR);
                }
            }
        }
    }
    else{
        "Failed to bind server, check port availability".log(LOGTYPE::ERROR); 
    }  
    println!("OKOKOOKOK------------------------------------------");  
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