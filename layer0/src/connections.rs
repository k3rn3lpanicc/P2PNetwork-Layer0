use std::{time::Duration, error::Error, sync::{Arc, Mutex}, collections::HashMap};
use colored::Colorize;
use std::net::TcpStream;
use crate::{logger::{LOGTYPE, Logger}, ppacket::PPacket, client::{self, show_connections, send_ppacket}, wlist::Wlist};


const CONNECTIONS_LEN :i32 = 8;
const _CONNECTION_TIME: u64 = 3;
const WAITING_LIST_LEN : usize = 10;

// address : ip:port -> TcpStream


#[derive(Clone)]
pub struct Connection{
    pub id : i32,
    pub ip : String,
    pub port : i64,
}

impl Connection{
    pub fn from_string(ipp : String) -> Connection{
        // ipp is like ip:port so we have to split it based on :
        let ip = ipp.split(':').collect::<Vec<&str>>()[0];
        let port = ipp.split(':').collect::<Vec<&str>>()[1];
        Connection{
            id: 0,
            ip: ip.to_string(),
            port: port.parse::<i64>().unwrap(),
        }
    }
}


lazy_static!{
    static ref CONS : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new())); 
    static ref PING_LIST : Arc<Mutex<HashMap<String,bool>>> = Arc::new(Mutex::new(HashMap::new())); // ip:port , bool
    pub static ref MSG_HASHES : Arc<Mutex<HashMap<String,u64>>> = Arc::new(Mutex::new(HashMap::new())); // Key is the hash, value is the time of message
    pub static ref PORT : Arc<Mutex<u64>> = Arc::new(Mutex::new(8080)); 
    pub static ref HARDCODED_LIST : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    pub static ref IS_HARDCODE : Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref WAITING_LIST : Arc<Mutex<Wlist>> = Arc::new(Mutex::new(Wlist::new(WAITING_LIST_LEN)));
    pub static ref TCP_CONS : Arc<Mutex<HashMap<String,TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn add_connection(ip : &str, port : i64) -> Result<() , Box<dyn Error>>{
    let mut con = CONS.lock()?;
    if con.contains(&format!("{}:{}",ip,port)) {
        return Err("Connection already exists!".into());
    }
    let count = con.len();
    if count as i32 == CONNECTIONS_LEN{
        drop(con);
        return Err("Connections are full!".into());
    }
    let value = format!("{}:{}",ip,port); 
    con.push(value);
    drop(con);
    Ok(())
}



pub fn get_connections() -> Vec<Connection>{
    let con = CONS.lock().unwrap();
    let mut cons : Vec<Connection> = Vec::new();
    for (cnt , ipp) in con.iter().enumerate(){
        let ip = ipp.split(':').collect::<Vec<&str>>()[0];
        let port = ipp.split(':').collect::<Vec<&str>>()[1];
        cons.push(Connection{
            id : cnt as i32,
            ip : ip.to_string(),
            port : port.parse::<i64>().unwrap(),
        });
    }
    drop(con);
    cons
}

pub fn change_state(con : &Connection , state : &str)->bool{
    let mut ping_list = PING_LIST.lock().unwrap();
    let key = format!("{}:{}",con.ip,con.port);
    if state == "ping"{
        if !ping_list.contains_key(&key){
            ping_list.insert(key,true);
            drop(ping_list);
            return true;
        }
    }else if state == "pong"{
        ping_list.remove(key.as_str());
        drop(ping_list);
        return true;
    }
    drop(ping_list);
    false
}
pub fn get_state(con : &Connection)->String{
    let ping_list = PING_LIST.lock().unwrap();
    let key = format!("{}:{}",con.ip,con.port);
    if ping_list.contains_key(&key){
        drop(ping_list);
        "Ping".to_string()
    }else{
        drop(ping_list);
        "Pong".to_string()
    }
}
pub fn get_connection_with_add(ip : &str , port : i64) -> Connection{
    let cons = CONS.lock().unwrap();
    for (cnt,ipp) in cons.iter().enumerate(){
        let ip_ = ipp.split(':').collect::<Vec<&str>>()[0];
        let port_ = ipp.split(':').collect::<Vec<&str>>()[1];
        if ip_ == ip && port_ == port.to_string().as_str(){
            drop(cons);
            return Connection{
                id : cnt as i32,
                ip : ip.to_string(),
                port,
            }
        }
    }   
    drop(cons);
    Connection{
        id : -1,
        ip : ip.to_string(),
        port,
    }
}
pub fn send_ping(con : &Connection) -> Result<bool , Box<dyn Error>>{
    let mut stream = TcpStream::connect(format!("{}:{}" , con.ip , con.port))?; //add timeout to this line todo
    let _s = client::send_ppacket(&mut stream, &PPacket::ping())?; //add timeout to this line todo
    if change_state(con, "Ping"){
        std::thread::sleep(Duration::from_millis(500));
        let state = get_state(con);
        if state == "Pong"{
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn check_connections(){
    let connections = get_connections();
    if connections.is_empty(){
        return;
    }
    for con in connections{
        format!("Sending Ping to ->> {}:{}" , con.ip.to_string().green() , con.port.to_string().green()).log(LOGTYPE::MORE_INFO);
       // let res = send_ping(&con).await.un;
        match send_ping(&con){
            Ok(b)=>{
                if b{
                    format!("Ping to {}:{} was successful", con.id,con.port).magenta().to_string().log(LOGTYPE::MORE_INFO);
                }
                else{
                    format!("Ping to {}:{} timed out", con.id,con.port).log(LOGTYPE::ERROR);
                    remove_connection(&con.ip , con.port);
                }
                
            },
            Err(_k) => {
                format!("Couldn't Ping {}:{}" , con.ip.to_string().red() , con.port.to_string().red()).log(LOGTYPE::ERROR);
                remove_connection(&con.ip , con.port);
                show_connections();
            }
        }
    }
}


pub fn send_connection_request() {
    let port : u64 = *PORT.lock().unwrap();
    let connections = get_connections();
    let packet : PPacket = PPacket::con_req("127.0.0.1", port as i64);
    if get_connections_len() != 0 {
        "Sending Connection Request to all Connections".log(LOGTYPE::INFO);
        for con in connections{
            format!("Sending Connection ReQ to {}:{}" , con.ip , con.port).log(LOGTYPE::INFO);
            send_message(&packet, &con);
        }
    }
    else{
        let hardcoded_list = HARDCODED_LIST.lock().unwrap().clone();
        for con in hardcoded_list{
            format!("Sending Connection ReQ to {}" , con).log(LOGTYPE::INFO);
            send_message(&packet, &Connection::from_string(con));
        }
        //todo : Send to hardcoded server
    }
}

pub fn is_connections_full()->bool {
    get_connections_len() >= CONNECTIONS_LEN
}

#[allow(dead_code)]
pub fn get_nth_connection(n : i32) -> Connection{
    let cons = CONS.lock().unwrap();
    let con = cons.iter().nth(n as usize).unwrap();
    let ip = con.split(':').collect::<Vec<&str>>()[0];
    let port = con.split(':').collect::<Vec<&str>>()[1];
    let conn = Connection{
        id : n,
        ip : ip.to_string(),
        port : port.parse::<i64>().unwrap(),
    };
    drop(cons);
    conn
}


pub fn remove_connection(ip : &str , port : i64){
    let mut cons = CONS.lock().unwrap();
    for (cnt,ipp) in cons.iter().enumerate(){
        if ipp == format!("{}:{}" , ip , port).as_str(){
            cons.remove(cnt);
            break;
        }
    }
    drop(cons);
}

pub fn get_connections_len() -> i32{
    let cons = CONS.lock().unwrap();
    let len = cons.len() as i32;
    drop(cons);
    len
}

pub fn send_message(packet : &PPacket , connection : &Connection ){
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}" , connection.ip , connection.port)){
        if let Ok(res) = client::send_ppacket(&mut stream, packet){
            if !res{
                remove_connection(&connection.ip , connection.port);
            }
        }
        else{
            remove_connection(&connection.ip,connection.port);   
        }
    }
    else{
        format!("Couldn't Connect to {}:{}" , connection.ip , connection.port).as_str().log(LOGTYPE::ERROR);
        remove_connection(&connection.ip,connection.port);
    }
}

pub fn get_my_ip() -> String{
    // let output = Command::new("curl").arg("ifconfig.me").output().unwrap();
    // String::from_utf8(output.stdout).unwrap()
    "127.0.0.1".to_string()
}

#[allow(dead_code)]
pub fn broadcast_message(packet : &PPacket) -> Result<bool , Box<dyn Error>>{
    let mut cons = TCP_CONS.lock().unwrap();
    for (_client_name , tcpcon) in cons.iter_mut(){    
        send_ppacket(tcpcon, packet)?;
    }
    Ok(true)
}
pub fn add_tcp_con(name : String , con : TcpStream){
    let mut cons = TCP_CONS.lock().unwrap();
    cons.insert(name, con);
    drop(cons);
}
pub fn get_my_port()->u64{
    let port = *PORT.lock().unwrap();
    port
}
