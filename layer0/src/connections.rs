use std::time::Duration;
use colored::Colorize;
use tokio::{net::TcpStream, stream};

use crate::{logger::{LOGTYPE, Logger}, ppacket::PPacket, client};
const CONNECTIONS_LEN :i32 = 8;
const CONNECTION_TIME: u64 = 3;



#[derive(Clone)]
pub struct Connection{
    pub id : i32,
    pub ip : String,
    pub port : i64,
}
pub struct Con{
    id : i32,
    ip : String,
    port : String,
}

pub fn con_to_connection(con : &Con)->Connection{
    Connection{
        id : con.id.to_owned(),
        ip : con.ip.to_string(),
        port : con.port.to_owned().parse::<i64>().unwrap(),
    }
}

pub fn get_connection()-> rusqlite::Connection{ rusqlite::Connection::open("hashes.db").unwrap() }

pub async fn clean_server(){
    get_connection().execute("DELETE FROM cons", []).unwrap();    
}

pub async fn add_connection(ip : &str, port : i8){
    if get_connections_len().await == CONNECTIONS_LEN {
        return;
    }
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons WHERE ip = ? AND port = ?").unwrap();
    let rows = stmt.query_map([ip , port.to_string().as_str()] , |_row|{Ok(1)}).unwrap();
    if rows.count() == 0{
        conn.execute("INSERT INTO cons (ind , ip , port) VALUES (? , ? , ?)", [get_next_index().to_owned().to_string().as_str(), ip , port.to_string().as_str()]).unwrap();
    }
    else{
        format!("Connection {} : {} already exists" , ip , port).log(LOGTYPE::ERROR);
    }
}



pub async fn get_connections() -> Vec<Connection>{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(con_to_connection(&row.unwrap()));
    }
    cons
}
pub fn change_state(con : &Connection , state : &str)->bool{
    if get_connection().execute("UPDATE cons set state = ? where ind = ?", [state , con.id.to_owned().to_string().as_str()]).is_ok(){
        return true;
    }
    false
}
pub fn get_state(con : &Connection)->String{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT state FROM cons WHERE ind = ?").unwrap();
    let rows = stmt.query_map([con.id.to_owned().to_string().as_str()] , |row|{
        Ok(row.get(0).unwrap())
    }).unwrap();
    for row in rows{
        return row.unwrap();
    }
    String::from("None")
}
pub fn get_connection_with_add(ip : &str , port : i64) -> Connection{
    let con = get_connection();
    let mut stmt = con.prepare("SELECT * FROM cons WHERE ip = ? AND port = ?").unwrap();
    let rows = stmt.query_map([ip , port.to_string().as_str()] , |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    for row in rows{
        return con_to_connection(&row.unwrap());
    }
    return Connection{
        id : -1,
        ip : "".to_string(),
        port : 0,
    }
}
pub async fn send_ping(con : &Connection) -> bool{
    if let Ok(stream) = tokio::time::timeout(Duration::from_secs(1) , TcpStream::connect(format!("{}:{}" , con.ip , con.port))).await{
        if let Ok(_s) = tokio::time::timeout(Duration::from_secs(1), client::send_ppacket(&mut stream.unwrap(), &PPacket::ping())).await{
            if change_state(con, "Ping"){
                format!("Sent Ping to {}:{} ... Waiting for Response (2secs)", con.id,con.port).log(LOGTYPE::INFO);
                tokio::time::sleep(Duration::from_secs(2)).await;
                let state = get_state(con);
                if state == "Pong"{
                    format!("Ping to {}:{} was successful", con.id,con.port).log(LOGTYPE::INFO);
                    return true;
                }
                else{
                    format!("Ping to {}:{} timed out", con.id,con.port).log(LOGTYPE::INFO);
                    return false;
                }
            }
            "Couldn't Change DB for Pinging!".log(LOGTYPE::ERROR);
            return false;
        }
        println!("Couldn't Connect to {}:{}" , con.ip , con.port);
        false
    }
    else{
        format!("Couldn't Connect to {}:{} -> Timeout" , con.ip , con.port).log(LOGTYPE::ERROR);
        false
    }
}

pub async fn check_connections(){
    let connections = get_connections().await;
    for con in connections{
        tokio::spawn(async move {
            format!("Sending Ping to ->> {}:{}" , con.ip.to_string().green() , con.port.to_string().green()).log(LOGTYPE::INFO);
            if !send_ping(&con).await{
                remove_connection(con.id , &con.ip , con.port).await;
            }
            else{
                println!("{}:{} is online" , con.ip , con.port);
            }
        });
    }
}

pub async fn send_connection_request() {
    let connections = get_connections().await;
    if get_connections_len().await != 0 {
        "Sending Connection Request to all Connections".log(LOGTYPE::INFO);
        let packet : PPacket = PPacket::from_str(1, format!("{{ \"ip\": \"{}\" , \"port\": \"{}\"}}" , "192.168.1.1" , "23").as_str());
        for con in connections{
            format!("Sending Connection ReQ to {}:{}" , con.ip , con.port).log(LOGTYPE::INFO);
            send_message(&packet, &con).await;
        }
    }
    else{
        //todo : Send to hardcoded server
    }
}

pub async fn is_connections_full()->bool {
    get_connections_len().await == CONNECTIONS_LEN
}

pub async fn get_nth_connection(n : i32) -> Connection{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons WHERE ind = ?").unwrap();
    let rows = stmt.query_map([n], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    return con_to_connection(cons.get(n as usize).unwrap());
}


pub async fn remove_connection(id : i32 , ip : &str , port : i64){
    let conn = get_connection();
    conn.execute("DELETE FROM cons WHERE ind = ?", [id.to_string()]).unwrap();
    format!("Connection Removed ->> {}:{}" , ip.to_string().yellow() , port.to_string().yellow()).log(LOGTYPE::ERROR);
}



pub async fn get_connections_len() -> i32{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    cons.len() as i32
}



pub fn get_next_index() -> i32{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons order by ind").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    for i in 0..cons.len(){
        if cons.get(i).unwrap().id!=(i+1) as i32{
            return (i+1) as i32;
        }
    }
    (cons.len()+1) as i32
}


pub async fn send_message(packet : &PPacket , connection : &Connection ){
    let mut stream : TcpStream;
    if let Ok(sr) = tokio::time::timeout(Duration::from_secs(CONNECTION_TIME),tokio::net::TcpStream::connect(format!("{}:{}" , connection.ip , connection.port))).await{
        stream = sr.unwrap();
    }
    else{
        "Connection timed out".log(LOGTYPE::ERROR);
        remove_connection(connection.id,&connection.ip , connection.port).await;
        return;
    }    
    println!("Sending message");
    if let Ok(res) = client::send_ppacket(&mut stream, packet).await{
        if !res{
            remove_connection(connection.id,&connection.ip , connection.port).await;
        }
    }
    else{
        remove_connection(connection.id,&connection.ip,connection.port).await;   
    }
}

