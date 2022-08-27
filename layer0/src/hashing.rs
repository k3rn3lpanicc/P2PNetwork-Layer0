
use crypto::digest::Digest;
use crypto::sha3::Sha3;

use rusqlite::{Connection, Result};
use std::time;

pub fn does_hash_exist(hash : &str)->bool{
    let conn = Connection::open("hashes.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM hashes WHERE msg_hash = ?").unwrap();
    let mut rows = stmt.query_map([hash] , |row|{Ok(1)}).unwrap();
    for row in rows{
        return true;
    }
    false
}
pub fn add_msg_hash(hash: &str){
    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    if let Ok(conn) = Connection::open("hashes.db"){
        if let Ok(_) = conn.execute("insert into hashes (msg_hash , date) values (?1, ?2)",    [hash , format!("{}",time).as_str()]){
            println!("\tInserted Message hash : {}" , hash);
        }
        else{
            println!("Already exists");
        }
        conn.close().unwrap();
    }
    else{
        println!("Failed to open database");
    }
}


pub fn get_hash(bytes : &[u8])->String{
    let mut hasher = Sha3::sha3_256();
    //hasher.input_str(bytes);
    hasher.input(bytes);    
    let result = hasher.result_str();
    result
}
pub fn get_hash_str(data : &str)->String{
    let mut hasher = Sha3::sha3_256();
    //hasher.input_str(bytes);
    hasher.input_str(data);
    let result = hasher.result_str();
    result
}
