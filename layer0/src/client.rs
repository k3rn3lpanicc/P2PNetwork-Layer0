use std::{fs, thread};
#[allow(unused_imports)]
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::net::TcpStream;
use crate::connections::{change_state, get_connection_with_add};
use crate::logger::{LOGTYPE, Logger};
use crate::{logger, connections};
use colored::Colorize;
use crate::hashing;
use crate::jsonize::{self, Jsonize};
use crate::ppacket::{PPacket, from_byte_vec};

#[allow(dead_code)]
pub fn save_hash_to_file(file_name : &str){
    let mut file = fs::File::create(file_name).unwrap();
    let hash = crate::hashing::get_hash_str("Hello World"); 
    file.write_all(hash.as_bytes()).unwrap();
}

pub fn send_ppacket(stream : &mut TcpStream, packet : &PPacket) -> Result<bool , &'static str>{
    let message = packet.to_byte_vec();
    if stream.write_all(&message).is_ok(){
        return Ok(true);
    }
    Err("Connection closed!")
}
pub fn read_ppacket(stream : &mut TcpStream)-> Result<PPacket,&'static str>{
    let mut message = vec![];
    let mut buf = [0; 1024];
    loop{
        let readed = stream.read(&mut buf).unwrap();
        message.extend_from_slice(&buf[0..readed]);
        if readed < 1024{
            break;
        }
    }
    if message.is_empty() {
        return Err("Connection Closed!");
    }
    from_byte_vec(&message)
}

pub fn show_connections(){
    if connections::get_connections_len()!=0{
        let mut to_show  = "Connections : ".bright_white().to_string()+"\n";
        let cons = connections::get_connections();
        for k in cons{
            to_show += format!("\t\t\t\t\t\t\t|{}:{}|" , k.ip.bright_green() , k.port.to_string().bright_magenta()).as_str();
            to_show += "\n";
        }
        to_show.log(LOGTYPE::INFO);
    }
}

fn broadcast_con_req_to_wlist(packet : &PPacket){
    let mut wlist = connections::WAITING_LIST.lock().unwrap();
    //TODO: check the order in loop
    for node in wlist.to_vec(){
        let mut stream = TcpStream::connect(node).unwrap();
        send_ppacket(&mut stream, &PPacket::con_ques()).unwrap();
        let result = read_ppacket(&mut stream).unwrap();
        if result.is_con_ans(){
            if result.get_ans(){
                
            }
            else{

                //TODO: wlist.remove_node();
                //remove the node from the list
            }
        }
        //TODO: we should check for them to see if we should keep them or remove them from list!
        
    }
}

fn handle_connection_request(packet : PPacket) {
    
    /*
        N will be 10 at first
        Waiting list : It's fix sized (DONE)
            new connection requests will be entered from left, and will be removed from right(when it reaches more than N requests). also we read them from right to left(most old item will be read first) 
        
        fn spread : send the connection request (first to the waiting list and then our neighbour nodes (except the incoming node))

        Does the node still wnats to connect?
            -yes :
                -do we have space for more connections? 
                    -yes :
                        -connect to that node and add it to our list
            -no :  
                End the proccess and do nothing (not even spread)
        Then spread the connection request to our neighbours and waiting list
        
        TODO : Implement a Waiting list(Done), Make a connection check mechanism (to see if they still have place for new connections),DDOS Attack Scenarios??
    */
    let payload = std::str::from_utf8(&packet.payload).unwrap();
    let json = jsonize::from_str(payload);
    // if it has the keys and it wasn't me (I don't want to connect to myself)
    if json.has_key("ip") && json.has_key("port") && format!("{}" , json.get_key("ip"))!=connections::get_my_ip() && format!("{}" , json.get_key("port"))!=connections::get_my_port().to_string(){
        thread::spawn(move || {
            let ip = json.get_key("ip");
            let port = json.get_key("port");
            let ipp = ip.as_str().unwrap(); 
            
            // new implementation
            let mut stream = TcpStream::connect(format!("{}:{}",ipp,port.as_str().unwrap())).unwrap();
            send_ppacket(&mut stream, &PPacket::con_ques()).unwrap();
            let packet = read_ppacket(&mut stream).unwrap();
            if packet.is_con_ans(){
                if !packet.get_ans(){
                    return;
                }
                else{
                    //connect to it
                    thread::spawn(move ||{
                        let con_ip = stream.peer_addr().unwrap().ip();
                        let con_port = stream.peer_addr().unwrap().port();
                        let client_name = format!("{}:{}",con_ip,con_port);
                        connections::add_tcp_con(client_name.to_string(), stream);
                        handle_client(client_name.as_str() , "client"); 
                    });
                    //add it to waiting list and spread it
                    let packet : PPacket = PPacket::con_income(&ip.to_string(), &port.to_string());
                    thread::spawn(move ||{
                        broadcast_con_req_to_wlist(&packet);
                    });
                }
            }
            //spread

            let cons = connections::get_connections();
            for k in cons{
                match TcpStream::connect(format!("{}:{}",k.ip,k.port)){
                    Ok(mut stream) => {
                        let packets = PPacket::new(1, json.to_string().as_bytes());
                        if send_ppacket(&mut stream, &packets).is_ok(){
                            hashing::add_msg_hash(&packets.overall_checksum());
                            format!("Bounced connection request to {}:{}" , k.ip , k.port).bright_yellow().to_string().log(LOGTYPE::INFO);
                        };
                    },
                    Err(e) => {
                        e.to_string().log(LOGTYPE::ERROR);
                    }
                }
            }    
            
            //here we should ask the node if it steel wants the connection or not, if it did, we should add it to our connections
            
            
            if let Err(err) = connections::add_connection(ipp, port.to_string().parse::<i64>().unwrap()){
                if format!("{}" , err) != "Connection already exists!"{
                    logger::log(format!("Error while adding connection : {}" , err).as_str(), logger::LOGTYPE::ERROR);
                }
            }
            else{
                format!("{} {}:{}" , "Connection added : ".bright_white() , ipp.to_string().green(),port.to_string().green()).log(LOGTYPE::INFO);
                show_connections();
            }
        });   
    }
    else{
        //PPacket is not valid (it has code 1 but it doesn't have ip and port)
    }

}

fn handle_ping_pong(packet : PPacket , client_id : &str){
    let mut tcons = connections::TCP_CONS.lock().unwrap();
    let stream = tcons.get_mut(client_id).unwrap();
    let ip = stream.peer_addr().unwrap().ip().clone();
    let port = stream.peer_addr().unwrap().port().clone();
    if packet.is_ping(){
        format!("{}" , "Received ping".green()).log(LOGTYPE::INFO);
        let packet = PPacket::pong();
        send_ppacket(stream, &packet).unwrap();
        drop(tcons);
        "Sending Pong".bright_green().to_string().log(LOGTYPE::INFO);
    }
    else if packet.is_pong() {
        drop(tcons);
        format!("{}" , "Received pong".green()).log(LOGTYPE::INFO);
        change_state(&get_connection_with_add(&ip.to_string() , port as i64), "Pong");
    }
}


pub fn handle_client(client_id : &str , _mode : &'static str){
    /*
    *   we should be able to send connection requests to other nodes without connecting *
    *   to them over and over again (with one tcpconnection), so we should keep         *
    *   connections open and also access them from different threads. But How?          *
    */
    loop{
        let mut cons = connections::TCP_CONS.lock().unwrap();
        let stream =  cons.get_mut(client_id).unwrap();
        let rpacket = read_ppacket(stream);
        drop(cons);
        show_connections();
        match rpacket{
            Ok(packet) => {
                if packet.is_valid(){
                    if !hashing::does_hash_exist(&packet.overall_checksum()){
                        logger::log(format!("Received command : {} , checksum : {}.. , payload : {}" , packet.command,&packet.checksum[0..16], std::str::from_utf8(&packet.payload).unwrap()).as_str(), logger::LOGTYPE::INFO);
                        hashing::add_msg_hash(&packet.overall_checksum());
                        match packet.command{
                            1 => {
                                handle_connection_request(packet);
                            },
                            2 => {
                                handle_ping_pong(packet,client_id);
                            },
                            3 => { 
                                
                            },
                            4 => {

                            },
                            _ => {
                                println!("Command not found!");
                            }
                        }
                    }
                    else {
                        logger::log("Message Hash already in database" , logger::LOGTYPE::MORE_INFO); //penalty
                    }
                }
                else{
                    logger::log("Invalid packet!", logger::LOGTYPE::ERROR); //penalty
                }
            },
            Err(err) =>{
                // if err == "Connection Closed!"{
                //     break;
                // }
                format!("Disconnected : {}" , err.bright_yellow()).log(LOGTYPE::ERROR);
                break;
            }
        }
    }
}

pub fn handle_application(stream : &mut TcpStream){
    let mut message = vec![];
    let mut buf = [0; 1024];
    loop{
        let readed = stream.read(&mut buf).unwrap();
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
                stream.write_all(b"Message sent").unwrap();
                //do something
            },
            "lol" => {
                stream.write_all(b"lol").unwrap();
                //do something
            },
            _ => {
                stream.write_all(b"Unknown command").unwrap();
                //do something
            }
        }       
        
    }
    else{
        stream.write_all(b"command does not exist").unwrap();
    }
}
