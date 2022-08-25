mod sha3;
use sha3::{get_hash_str, get_hash};

fn main() {
    println!("Sha3('abc') = {}", get_hash_str("abc"));
    println!("Sha3('abc') = {}", get_hash("abc".as_bytes())); // same as above
}
