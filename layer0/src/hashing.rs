

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rusqlite::Connection;
use std::time;
use crate::logger::{Logger , LOGTYPE};

pub async fn hash_remover(){
    let conn = Connection::open("hashes.db").unwrap();
    loop{
        format!("Hash remover is removing hashes that are more than 1 minute old.").as_str().log(LOGTYPE::INFO); 
        if let Ok(_) = conn.execute("DELETE FROM hashes WHERE date < ?", [format!("{}" , time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs())]){
            //println!("Hashes removed");
        }
        else{
            println!("Failed to remove hashes");
        }
        std::thread::sleep(time::Duration::from_secs(60));
    }
}
pub fn does_hash_exist(hash : &str)->bool{
    let conn = Connection::open("hashes.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM hashes WHERE msg_hash = ?").unwrap();
    let rows = stmt.query_map([hash] , |_row|{Ok(1)}).unwrap();
    rows.count()>0
}
pub fn add_msg_hash(hash: &str){
    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    if let Ok(conn) = Connection::open("hashes.db"){
        if let Ok(_) = conn.execute("insert into hashes (msg_hash , date) values (?1, ?2)",    [hash , format!("{}",time).as_str()]){
            format!("Inserted Message hash : {}" , hash).as_str().log(LOGTYPE::INFO);
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
#[allow(dead_code)]
pub fn get_hash_str(data : &str)->String{
    let mut hasher = Sha3::sha3_256();
    //hasher.input_str(bytes);
    hasher.input_str(data);
    let result = hasher.result_str();
    result
}
