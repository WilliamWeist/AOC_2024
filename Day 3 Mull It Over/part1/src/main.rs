use std::fs::File;
use linereader::LineReader;
use regex::Regex;

fn main() {
    let myfile: &str = "input.txt";
    let input: &str = &file_to_string(myfile);
    let mul_regex: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut result = 0;
    for match_found in mul_regex.find_iter(input) {
        let text = input[match_found.start()..match_found.end()].replace("mul(", "").replace(")", "");
        let values = text.split(",");
        let mut multiplication = 1;
        for value in values {
            multiplication = multiplication * value.parse::<i32>().unwrap();
        }
        result = result + multiplication;
    }
    println!("{result}");
}

fn file_to_string(path: &str) -> String {
    let mut content: String = String::new();
    let file = File::open(path).expect("open");
    let mut reader = LineReader::new(file);
    while let Some(b_line) = reader.next_line() {
        let b_line = b_line.expect("read error");
        let s_line = match std::str::from_utf8(b_line) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        content.push_str(s_line);
    }
    return content
}
