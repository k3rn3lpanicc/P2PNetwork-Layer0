use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::time;
use crate::{logger::{Logger , LOGTYPE}, connections::{self, check_connections}};

pub fn hash_remover(){
    loop{
        check_connections();
        let number_of_cons = connections::get_connections_len();
        let is_hardcode = connections::IS_HARDCODE.lock().unwrap().clone();
        if  number_of_cons < 8 && !is_hardcode{
            connections::send_connection_request();
        }
        remove_hashes();
        "Hash remover removed hashes that are more than 1 minute old".log(LOGTYPE::MORE_INFO); 
        std::thread::sleep(time::Duration::from_secs(10));
    }
}
fn remove_hashes(){
    let mut hashes = connections::MSG_HASHES.lock().unwrap();
    let mut hashes_to_remove = Vec::new();
    for (key , value) in hashes.iter(){
        if value < &time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(){
            hashes_to_remove.push(key.clone());
        }
    }
    for hash in hashes_to_remove{
        hashes.remove(&hash);
    }
    drop(hashes);
}
pub fn does_hash_exist(hash : &str)->bool{
    let hashes = connections::MSG_HASHES.lock().unwrap();
    let res = hashes.contains_key(&hash.to_string());
    drop(hashes);
    res
}
pub fn add_msg_hash(hash: &str){
    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
    if does_hash_exist(hash){
        return;
    }
    let mut hashes = connections::MSG_HASHES.lock().unwrap();
    hashes.insert(hash.to_string() , time);
    format!("Inserted Message hash : {}.." , &hash[0..16]).as_str().log(LOGTYPE::INFO);
    drop(hashes);
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
