mod hashing;
mod ppacket;
mod client;
mod jsonize;
mod connections;
mod hardcoded;
mod wlist;
use client::{read_ppacket, send_ppacket};
use ppacket::PPacket;
use std::thread;
use std::net::TcpListener;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::{env, error::Error};
use colored::Colorize;
use crate::logger::Logger;
mod logger;
use logger::LOGTYPE;
#[allow(dead_code, unused_imports)]

pub fn get_hardcoded_servers() -> Vec<String>{
    let hardcoded_servers = connections::HARDCODED_LIST.lock().unwrap();
    hardcoded_servers.clone().to_vec()
}

const APPLICATION_PORT : u16 = 48134;
const PORT_NUMBER : i64 = 8080;


#[macro_use]
extern crate lazy_static;

fn main() {
    // 1: cargo run 2>/dev/null port hardcode 2: cargo run 2>/dev/null port
    env::set_var("RUST_BACKTRACE", "1");
    
    // read hardcoded addresses from source file to connect them in the beginning
    hardcoded::set_hardcoded_servers_from_file("hsources".to_string());
    format!("Hardcoded Servers : {:?}" , get_hardcoded_servers()).bright_magenta().to_string().log(LOGTYPE::INFO);
    
    //println!("Public IP : {}" , connections::get_my_ip());
    
    // read the arguments from the command line
    let args: Vec<String> = env::args().collect();

        // get the mode of running (hardcode node or normal client)
        let mode = if args.len()>2 && args[2] == "hardcode" { "hardcode" } else { "client" };
        let mut is_hardcode = connections::IS_HARDCODE.lock().unwrap();
        *is_hardcode = mode == "hardcode";
        drop(is_hardcode);

        // Set the port this Node's running on
        let port_number_var : i64 = if args.len()>1 { args[1].parse::<i64>().unwrap() } else { PORT_NUMBER }; 
        let mut port = connections::PORT.lock().unwrap();
        *port = port_number_var as u64;
        drop(port);

    format!("Node's Mode : {}",mode.bright_white()).log(LOGTYPE::DEBUG);

    // create a listener
    let th1 = thread::spawn(move || {
        if let Err(err) = server_handle(mode,port_number_var as u16){
            format!("Server failed! : {}",err).log(LOGTYPE::ERROR);
        }
    });
    // create a client handler
    let th2 = thread::spawn(move || {
        if let Err(err) = application_handle(){
            format!("Application failed! : {}",err).log(LOGTYPE::ERROR);
        }
    });
    
    
    // send a connection request to the hardcoded servers
        // if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080"){
        //     for _i in 8081..8086{
        //         std::thread::sleep(std::time::Duration::from_millis(250));    
        //         let packet : PPacket = PPacket::from_str(1 , format!("{{\"ip\":\"127.0.0.1\" , \"port\":\"{}\"}}" , _i).as_str());
        //         //format!("overall checksum : {}", packet.overall_checksum()).log(LOGTYPE::DEBUG);
        //         match client::send_ppacket(&mut stream, &packet){
        //             Ok(_b) =>{
        //                 format!("Sent {}.." , &packet.overall_checksum()[0..16]).as_str().log(LOGTYPE::INFO);
        //             },
        //             Err(err) =>{
        //                 format!("Error sending packet : {}" , err).log(LOGTYPE::ERROR);
        //             }
        //         }
        //     }
        // }
        // else{
        //     "Couldn't connect to server".log(LOGTYPE::ERROR);
        // }
    th1.join().unwrap();
    th2.join().unwrap();
}


fn server_handle(mode :  &'static str , port_number : u16) -> Result<() , Box<dyn Error>>{
    let address : String = format!("{}:{}" , "0.0.0.0" , port_number);
    
    let listener = TcpListener::bind(address)?;
    format!("Listening on Server side : {}:{}" , "0.0.0.0".bright_red().underline() , port_number.to_string().bright_red().underline()).log(LOGTYPE::INFO);
    // spawn the hash_remover and connection checker thread
    thread::spawn(move || {
        hashing::hash_remover();
    });
    
    loop{
        //"Waiting for connection..".bright_red().to_string().log(LOGTYPE::INFO);
        let (mut stream , _address) = listener.accept()?;
        //con_req , con_ques , con_ans
        let initial_packet = read_ppacket(&mut stream)?;
        if initial_packet.is_con_ques(){
            format!("Connection Question from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO); 
            send_ppacket(&mut stream, &PPacket::con_ans(!connections::is_connections_full()))?;
        }
        else if initial_packet.is_con_req(){
            format!("Connection Request from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO); 
            //this part can be sended to client application for more control ?! dunno
            if !connections::is_connections_full(){
                let client_name = format!("{}:{}" , stream.peer_addr()?.ip() , stream.peer_addr()?.port());
                let mut cons = connections::TCP_CONS.lock()?;
                cons.insert(client_name.clone(), stream);
                format!("New connection from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO); 
                thread::spawn(move || {
                    client::handle_client(client_name.as_str() , mode);
                });
            }
        }
        else{
            //it didn't started with connection request or question!
        }
    }
}

fn application_handle() -> Result<() , Box<dyn Error>>{
    let address : String = format!("{}:{}" , "127.0.0.1" , APPLICATION_PORT);
    let listener = TcpListener::bind(address)?;
    "Listening on application side".bright_red().to_string().log(LOGTYPE::INFO);
    loop{
        let (mut stream , _address) = listener.accept().unwrap();
        format!("New connection from {}", _address.to_string().bright_magenta()).log(LOGTYPE::INFO);
        thread::spawn( move ||{
            client::handle_application(&mut stream);
        });
    }
}