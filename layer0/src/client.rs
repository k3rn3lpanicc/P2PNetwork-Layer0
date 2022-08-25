use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::ppacket::PPacket;

pub async fn send_ppacket(stream : &mut TcpStream, packet : &PPacket){
    let mut message = vec![];
    message.extend_from_slice(&packet.command.to_le_bytes());
    message.extend_from_slice(&packet.payload_size.to_le_bytes());
    message.extend_from_slice(&packet.checksum.as_bytes());
    message.extend_from_slice(&packet.payload);
    //println!("{} bytes sent", message.len());
    stream.write_all(&message).await.unwrap();
}
pub async fn read_ppacket(stream : &mut TcpStream)->PPacket{
    let mut message = vec![];
    let readed = stream.read_to_end(&mut message).await.unwrap();
    //println!("Readed {} bytes", readed);
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