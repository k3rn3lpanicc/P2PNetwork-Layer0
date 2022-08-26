use std::fs::OpenOptions;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use crate::jsonize::{self, Jsonize};
use crate::ppacket::{PPacket, from_byte_vec};

pub async fn save_hash_to_file(file_name : &str){
    let mut file = fs::File::create(file_name).await.unwrap();
    let hash = crate::hashing::get_hash_str("Hello World");
    
    file.write_all(hash.as_bytes()).await.unwrap();
}

pub async fn send_ppacket(stream : &mut TcpStream, packet : &PPacket){
    let message = packet.to_byte_vec();
    stream.write_all(&message).await.unwrap();
}
pub async fn read_ppacket(stream : &mut TcpStream)->PPacket{
    let mut message = vec![];
    let mut buf = [0; 1024];
    loop{
        let readed = stream.read(&mut buf).await.unwrap();
        message.extend_from_slice(&buf[0..readed]);
        if readed < 1024{
            break;
        }
    }
    from_byte_vec(&message)
}
pub async fn handle_client(stream : &mut TcpStream){
    //here we will read the ppackets and proccess them
    loop{
        let packet : PPacket = read_ppacket(stream).await;
        if packet.is_valid(){
            println!("Received command : {} , payload_size : {} , checksum : {} , payload : {:?} , payload in str format : {:?}" , packet.command,packet.payload_size,packet.checksum,packet.payload , std::str::from_utf8(&packet.payload).unwrap());
        }
        else{
            println!("Invalid packet");
        }
    }
}

pub async fn handle_application(stream : &mut TcpStream){
    let mut message = vec![];
    let mut buf = [0; 1024];
    loop{
        let readed = stream.read(&mut buf).await.unwrap();
        message.extend_from_slice(&buf[0..readed]);
        if readed < 1024{
            break;
        }
    }
    let st:String = message.into_iter().map(|x| x as char).collect::<String>().replace("'","\"");
    let jsoned = jsonize::from_str(&st);
    if jsoned.has_key("command"){
        let command = jsoned.get_key("command").to_string();
        println!("command : {}", command);
        match command.as_str(){
            "sendMessage" => {
                stream.write_all(b"Message sent").await.unwrap();
                //do something
            },
            "lol" => {
                stream.write_all(b"lol").await.unwrap();
                //do something
            },
            _ => {
                stream.write_all(b"Unknown command").await.unwrap();
                //do something
            }
        }        
    }
    else{
        stream.write_all(b"command does not exist").await.unwrap();
    }
}