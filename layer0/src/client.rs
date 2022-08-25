use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::jsonize::{self, Jsonize};
use crate::ppacket::PPacket;

pub async fn send_ppacket(stream : &mut TcpStream, packet : &PPacket){
    let mut message = vec![];
    message.extend_from_slice(&packet.command.to_le_bytes());
    message.extend_from_slice(&packet.payload_size.to_le_bytes());
    message.extend_from_slice(&packet.checksum.as_bytes());
    message.extend_from_slice(&packet.payload);
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
    let command = u64::from_le_bytes(message[0..8].try_into().unwrap());
    let payload_size = u32::from_le_bytes(message[8..12].try_into().unwrap());
    let checksum = String::from_utf8(message[12..(message.len()-payload_size as usize)].to_vec()).unwrap();
    let payload = message[(message.len()-payload_size as usize)..].to_vec();
    //println!("command {} payload_size {} checksum {} payload size {}", command, payload_size, checksum, payload.len());
    PPacket{
        command,
        payload_size,
        checksum,
        payload,
    }
}
pub async fn handle_client(stream : &mut TcpStream){
    //here we will read the ppackets and proccess them
    loop{
        let packet : PPacket = read_ppacket(stream).await;
        println!("Received command : {} , payload_size : {} , checksum : {} , payload : {:?} , payload in str format : {}" , packet.command,packet.payload_size,packet.checksum,packet.payload , std::str::from_utf8(&packet.payload).unwrap());
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