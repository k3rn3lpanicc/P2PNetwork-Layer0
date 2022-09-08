#[allow(unused_imports)]
use std::fs::OpenOptions;
use crate::connections::{change_state, Connection, get_connection_with_add};
use crate::logger::{LOGTYPE, Logger};
use crate::{logger, connections};
use colored::Colorize;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use crate::hashing;
use crate::jsonize::{self, Jsonize};
use crate::ppacket::{PPacket, from_byte_vec};
#[allow(dead_code)]
pub async fn save_hash_to_file(file_name : &str){
    let mut file = fs::File::create(file_name).await.unwrap();
    let hash = crate::hashing::get_hash_str("Hello World"); 
    file.write_all(hash.as_bytes()).await.unwrap();

}

pub async fn send_ppacket(stream : &mut TcpStream, packet : &PPacket) -> Result<bool , &'static str>{
    let message = packet.to_byte_vec();
    if stream.write_all(&message).await.is_ok(){
        return   Ok(true);
    }
    Err("Connection closed!")
}
pub async fn read_ppacket(stream : &mut TcpStream)->Result<PPacket,&'static str>{
    let mut message = vec![];
    let mut buf = [0; 1024];
    loop{
        let readed = stream.read(&mut buf).await.unwrap();
        message.extend_from_slice(&buf[0..readed]);
        if readed < 1024{
            break;
        }
    }
    if message.is_empty() {
        return Err("Connection Closed!");
    }
    Ok(from_byte_vec(&message))
}

pub async fn show_connections(){
    if connections::get_connections_len().await!=0{
        "Connections : ".log(LOGTYPE::DEBUG);
        let cons = connections::get_connections().await;
        for k in cons{
            println!("\t\t\t\t\t\t\t|{}:{}|" , k.ip.bright_green() , k.port.to_string().bright_magenta());
        }
    }
}

pub async fn handle_client(stream : &mut TcpStream , mode : &'static str){
    //!!!todo : add a ping-pong mechanism to handle the incoming packet just inside this function!!!
    //todo : if user didn't send a ppacket in a amount of time, close the connection
    loop{
        show_connections().await;
        if let Ok(packet ) = read_ppacket(stream).await{
            if packet.is_valid(){
                if !hashing::does_hash_exist(&packet.overall_checksum()){
                    logger::log(format!("Received command : {} , checksum : {}.. , payload : {}" , packet.command,&packet.checksum[0..16], std::str::from_utf8(&packet.payload).unwrap()).as_str(), logger::LOGTYPE::INFO);
                    match packet.command{
                        1 => {
                            let payload = std::str::from_utf8(&packet.payload).unwrap();
                            let json = jsonize::from_str(payload);
                            //println!("Received json : {}" , json);
                            if json.has_key("ip"){
                                let ip = json.get_key("ip");
                                let port = json.get_key("port");
                                let ipp = ip.as_str().unwrap();
                                connections::add_connection(ipp, port.to_string().parse().unwrap()).await;
                                format!("{} {}:{}" , "Connection added : ".bright_white() , ip.to_string().green(),port.to_string().green()).log(LOGTYPE::INFO);
                            }   
                        },
                        2 => {
                            if packet.is_ping(){
                                format!("{}" , "Received ping".green()).log(LOGTYPE::INFO);
                                let packet = PPacket::pong();
                                send_ppacket(stream, &packet).await.unwrap();
                                break;
                            }
                            else if packet.is_pong() {
                                format!("{}" , "Received pong".green()).log(LOGTYPE::INFO);
                                change_state(&get_connection_with_add(stream.peer_addr().unwrap().ip().to_string().as_str(), stream.peer_addr().unwrap().port() as i64), "Pong");
                                break;
                            }
                            
                        },
                        _ => {
                            println!("Command not found!");
                        }
                    }
                    hashing::add_msg_hash(&packet.overall_checksum());
                    break;
                }
                else {
                    logger::log("the message is already in the database" , logger::LOGTYPE::ERROR);
                }
            }
            else{
                logger::log("Invalid packet!", logger::LOGTYPE::ERROR);
            }
        }
        else{
            "Disconnected!".log(LOGTYPE::ERROR);
            break;
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