use std::fs::File;
use linereader::LineReader;
use regex::Regex;

fn main() {
    let myfile = "input.txt";
    let mut lvec: Vec<i32> = Vec::new();
    let mut rvec: Vec<i32> = Vec::new();
    
    let file = File::open(myfile).expect("open");
    let mut reader = LineReader::new(file);

    let mut entry = 0;
    let regex = Regex::new(r"\d+").unwrap();
    while let Some(b_line) = reader.next_line() {
        // Read utf-8 byte
        let b_line = b_line.expect("read error");
        // Covert to String
        let s_line = match std::str::from_utf8(b_line) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        // Split numbers into respecting vector using regex
        for match_found in regex.find_iter(s_line) {
            if entry % 2 == 0 {
                lvec.push(s_line[match_found.start()..match_found.end()].parse::<i32>().unwrap());
            } else {
                rvec.push(s_line[match_found.start()..match_found.end()].parse::<i32>().unwrap());
            }
            entry += 1;
        }
    }

    lvec.sort();
    rvec.sort();
    let mut result = 0;
    for x in 0..=lvec.len()-1 {
        result = result + (rvec[x] - lvec[x]).abs();
    }
    println!("{result}");
}
