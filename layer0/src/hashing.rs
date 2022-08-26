
use crypto::digest::Digest;
use crypto::sha3::Sha3;


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
