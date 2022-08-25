use std::ptr::null;

use crate::ppacket::PPacket;
use json::{self, object, JsonValue};

pub fn serialize_ppacket(packet : &PPacket)->String{
    serde_json::to_string(&packet).unwrap()
}
pub fn deserialize_ppacket(packet : &str)->PPacket{
    serde_json::from_str(packet).unwrap()
}
pub fn from_str(data : &str)->JsonValue{
    json::parse(data).unwrap()
}
pub trait Jsonize{
    fn get_key(&self , key : &str)->JsonValue;
    fn get_path(&self , path : &str)->JsonValue;
    fn has_key(&self , key : &str)->bool;
    fn has_path(&self , path : &str)->bool;
}

impl Jsonize for JsonValue{
    fn get_key(&self , key : &str)->JsonValue{
        self[key].clone()
    }
    fn get_path(&self , path : &str)->JsonValue{
        let mut json : JsonValue = self.clone();        
        let iter = path.split("->").collect::<Vec<&str>>();
        for i in 0..iter.len(){
            json = json[iter[i]].clone();
        }
        json
    }
    fn has_key(&self , key : &str)->bool{
        self[key].is_object()
    }
    fn has_path(&self , path : &str)->bool{
        let mut json : JsonValue = self.clone();        
        let iter = path.split("->").collect::<Vec<&str>>();
        for i in 0..iter.len(){
            if !json.has_key(iter[i]){
                //println!("{} not found in {}", iter[i],json);
                return false;
            }
            json = json[iter[i]].clone();
        }
        true
    }
}
