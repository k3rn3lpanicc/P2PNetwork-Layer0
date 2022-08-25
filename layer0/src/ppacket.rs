use crate::hashing;

#[derive(Serialize, Deserialize, Debug)]
pub struct PPacket{
    pub command : u64,
    pub payload_size : u32,
    pub checksum : String,
    pub payload : Vec<u8>,
}
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

}