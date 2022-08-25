mod hashing;
mod ppacket;
use hashing::{get_hash_str, get_hash};
use ppacket::PPacket;
fn main() {
    println!("Sha3('abc') = {}", get_hash_str("abc"));
    println!("Sha3('abc') = {}", get_hash("abc".as_bytes())); // same as above
    let a : PPacket = PPacket::from_str(1, "salam");
    let b : PPacket = PPacket::new(1, "salam".as_bytes());
    println!("A's checksum : {}" , a.checksum);
    println!("B's checksum : {}" , b.checksum);
}
