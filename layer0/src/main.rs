mod hashing;
mod ppacket;
mod client;
use ppacket::PPacket;
use tokio::net::{TcpListener, TcpStream};
use std::time::Duration;

const PORT_NUMBER : u16 = 1234;
#[tokio::main]
async fn main() {
    let address : String = format!("{}:{}" , "0.0.0.0" , PORT_NUMBER);
    if let Ok(listener) = TcpListener::bind(address).await{ 
        println!("Listening on ");
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
        println!("Failed to bind, check port availability"); 
    }    
    let mut stream : TcpStream = TcpStream::connect("0.0.0.0:1234").await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    client::send_ppacket(&mut stream, &PPacket::from_str(123,"abc")).await;
    println!("Sent {}" , PPacket::from_str(123,"abc").get_checksum());
    
    
}
