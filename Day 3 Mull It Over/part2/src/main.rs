use std::fs::File;
use linereader::LineReader;
use regex::Regex;

fn main() {
    let myfile: &str = "input.txt";
    let input: &str = &file_to_string(myfile);
    let sanitized_input: &str = &remove_dont(input);
    let mul_regex: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut result = 0;
    for match_found in mul_regex.find_iter(sanitized_input) {
        let text = sanitized_input[match_found.start()..match_found.end()].replace("mul(", "").replace(")", "");
        let values = text.split(",");
        let mut multiplication = 1;
        for value in values {
            multiplication = multiplication * value.parse::<i32>().unwrap();
        }
        result = result + multiplication;
    }
    println!("{result}");
}

fn remove_dont(input: &str) -> String {
    let mut to_parse: String = input.to_string();
    let mut output: String = String::new();
    let dont_regex: Regex = Regex::new(r"don't\(\).*").unwrap();
    let do_regex: Regex = Regex::new(r"do\(\).*").unwrap();
    let mut dont_section: bool = false;
    while to_parse.len() > 0 {
        let temp = to_parse.clone();
        if !dont_section {
            if dont_regex.is_match(&temp) {
                let match_found = dont_regex.find(&temp).unwrap();
                output.push_str(&to_parse[..match_found.start()]);
                to_parse = to_parse[match_found.start()..to_parse.len()].to_string();
                dont_section = true;
            } else {
                // No don't section in the input
                output.push_str(&to_parse[..]);
                to_parse = "".to_string();
            }
        } else {
            if do_regex.is_match(&temp) {
                let match_found = do_regex.find(&temp).unwrap();
                to_parse = to_parse[match_found.start()..to_parse.len()].to_string();
                dont_section = false;
            } else {
                to_parse = "".to_string();
            }
        }
    }
    return output
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
