use crate::connections::HARDCODED_LIST;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn set_hardcoded_servers_from_file(file_name : String){
    //read hardcoded server's addresses from a file and set it to it's refrence
    let mut ads = Vec::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                ads.push(ip);
            }
        }
    }
    let mut hcons = HARDCODED_LIST.lock().unwrap();
    for address in ads{
        hcons.push(address);
    }
    drop(hcons);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}