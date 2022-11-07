use crate::{hashing, jsonize::{self, Jsonize}};

#[derive(Serialize, Deserialize, Debug)]
pub struct PPacket{
    pub command : u64,
    pub payload_size : u32,
    pub checksum : String,
    pub payload : Vec<u8>,
}

pub fn from_byte_vec(bytes : &[u8])->Result<PPacket , &'static str>{
    if bytes.len()<12{
        return Err("Corrupted packet! : less than 12 bytes");
    }
    let command = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    let payload_size = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    if payload_size > bytes.len() as u32{
        return Err("Corrupted packet! : payload size is bigger than the packet size");
    }
    if (bytes.len() as u32 - payload_size) <= 12 {
        return Err("Corrupted packet! : there are less than 12 bytes in packet!");
    }
    let checksum = String::from_utf8(bytes[12..(bytes.len() as u32 -payload_size) as usize].to_vec()).unwrap();
    let payload = bytes[(bytes.len() as u32-payload_size as u32) as usize..].to_vec();
    if payload.len() as u32  != payload_size as u32{
        return Err("Corrupted packet! : payload size is not equal to the payload_size");
    }
    Ok(PPacket{
        command,
        payload_size,
        checksum,
        payload,
    })
}
#[allow(dead_code)]
impl PPacket{
    pub fn new(command : u64, payload : &[u8])->PPacket{
        let payload_size = payload.len() as u32;
        let checksum = hashing::get_hash(payload);
        PPacket{
            command,
            payload_size,
            checksum,
            payload: payload.to_vec(),
        }
    }
    pub fn ping()->PPacket{
        PPacket::new(2, b"Ping")
    }
    pub fn pong()->PPacket{
        PPacket::new(2, b"Pong")
    }
    pub fn con_req(my_ip : &str , my_port : i64)->PPacket{
        PPacket::new(1, format!("{{\"ip\":\"{}\",\"port\":\"{}\"}}" , my_ip , my_port).as_bytes())
    }
    pub fn req_ans(answer : bool) -> PPacket{
        PPacket::new(5, format!("{{\"ans\":\"{}\"}}" , answer).as_bytes())
    }
    pub fn is_req_ans(&self) -> bool {
        return self.command == 5;
    }
    pub fn is_con_req(&self) ->bool{
        return self.command == 1;
    }
    pub fn con_ques()->PPacket{
        PPacket::new(3, b"")
    }
    pub fn con_ans(ans : bool)->PPacket{
        PPacket::new(4, format!("{{\"res\":\"{}\"}}" , ans).as_bytes())
    }
    pub fn get_ans(&self) -> bool{
        let json = String::from_utf8(self.payload.clone()).unwrap();
        let json = jsonize::from_str(&json);
        let res = json.get_key("res").as_bool().unwrap();
        res
    }
    pub fn is_con_ans(&self)->bool{
        self.command == 4
    }
    pub fn is_con_ques(&self)->bool{
        self.command == 3
    }
    pub fn is_ping(&self)->bool{
        self.command == 2 && self.payload == b"Ping"
    }
    pub fn is_pong(&self)->bool{
        self.command == 2 && self.payload == b"Pong"
    }
    pub fn from_str(command : u64, payload : &str)->PPacket{
        let payload_size = payload.len() as u32;
        let checksum = hashing::get_hash(payload.as_bytes());
        PPacket{
            command,
            payload_size,
            checksum,
            payload: payload.as_bytes().to_vec(),
        }
    }
    pub fn get_checksum(&self)->String{
        hashing::get_hash(&self.payload)
    }
    pub fn is_valid(&self)->bool{
        self.checksum == self.get_checksum()
    }
    pub fn to_byte_vec(&self)->Vec<u8>{
        let mut message = vec![];
        message.extend_from_slice(&self.command.to_le_bytes());
        message.extend_from_slice(&self.payload_size.to_le_bytes());
        message.extend_from_slice(self.checksum.as_bytes());
        message.extend_from_slice(&self.payload);
        message
    }
    pub fn overall_checksum(&self)->String{
        hashing::get_hash(&(self.to_byte_vec()))
    }

}
