use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::time;
use crate::{logger::{Logger , LOGTYPE}, connections::{self, check_connections}};

pub async fn hash_remover(){
    loop{
        std::thread::sleep(time::Duration::from_secs(10));
        check_connections().await;
        "Checked for connections to remove..".log(LOGTYPE::DEBUG);
        let number_of_cons = connections::get_connections_len().await;
        if  number_of_cons < 8{
            connections::send_connection_request().await;
        }
        "Hash remover is removing hashes that are more than 1 minute old".log(LOGTYPE::INFO); 
        let conn = connections::get_connection();
        if let Err(err) =  conn.execute("DELETE FROM hashes WHERE date < ?", [format!("{}" , time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs())]){
            format!("Error removing hashes : {}", err).log(LOGTYPE::ERROR);
        }
    }
}
pub fn does_hash_exist(hash : &str)->bool{
    let conn = connections::get_connection();
    let mut stmt = conn.prepare("SELECT * FROM hashes WHERE msg_hash = ?").unwrap();
    let rows = stmt.query_map([hash] , |_row|{Ok(1)}).unwrap();
    rows.count()>0
}
pub fn add_msg_hash(hash: &str){
    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    let conn = connections::get_connection();
    if conn.execute("insert into hashes (msg_hash , date) values (?1, ?2)",    [hash , format!("{}",time).as_str()]).is_ok(){
        format!("Inserted Message hash : {}.." , &hash[0..16]).as_str().log(LOGTYPE::INFO);
    }
    else{
        println!("Already exists");
    }
    conn.close().unwrap();
}


pub fn get_hash(bytes : &[u8])->String{
    let mut hasher = Sha3::sha3_256();
    hasher.input(bytes);    
    hasher.result_str()
}
#[allow(dead_code)]
pub fn get_hash_str(data : &str)->String{
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(data);
    hasher.result_str()
}
