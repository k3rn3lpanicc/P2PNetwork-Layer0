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


fn get_hardcoded_servers() -> Vec<&'static str>{
    let hardcoded_addresses : Vec<&str> = vec!["127.0.0.1:1234" , "127.0.0.1:3214"];
    hardcoded_addresses
}

const APPLICATION_PORT : u16 = 48134;
const PORT_NUMBER : u16 = 1234;
#[tokio::main]
async fn main() {
    tokio::spawn(async move{
        server_handle().await;
    });
    tokio::spawn(async move{
        application_handle().await;
    });
    let packet : PPacket = PPacket::from_str(1 , "I have a message for you!");
    println!("overall checksum : {}", packet.overall_checksum());
    let mut stream : TcpStream = TcpStream::connect("0.0.0.0:1234").await.unwrap();
    //tokio::time::sleep(Duration::from_secs(3)).await;
    client::send_ppacket(&mut stream, &packet).await;
    println!("Sent {}" , packet.get_checksum());
    loop{}
}

async fn server_handle(){
    let address : String = format!("{}:{}" , "0.0.0.0" , PORT_NUMBER);
        if let Ok(listener) = TcpListener::bind(address).await{ 
            println!("Listening on Server side");
            loop{
                let (mut stream , _address) = listener.accept().await.unwrap();
                println!("New connection from {}", _address);
                tokio::spawn(async move {
                    client::handle_client(&mut stream).await;
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