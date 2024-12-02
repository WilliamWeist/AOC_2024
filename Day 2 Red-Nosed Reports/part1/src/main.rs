use std::fs::File;
use linereader::LineReader;
use regex::Regex;

fn main() {
    let myfile = "input.txt";
    let mut reports: Vec<Vec<i32>> = Vec::new();
    
    let file = File::open(myfile).expect("open");
    let mut reader = LineReader::new(file);

    let mut report = 0;
    let regex = Regex::new(r"\d+").unwrap();
    while let Some(b_line) = reader.next_line() {
        let b_line = b_line.expect("read error");
        let s_line = match std::str::from_utf8(b_line) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        reports.push(Vec::new());
        for match_found in regex.find_iter(s_line) {
            reports[report].push(s_line[match_found.start()..match_found.end()].parse::<i32>().unwrap());
        }
        report += 1;
    }

    let mut safe_report = 0;
    for x in 0..=reports.len()-1 {
        let mut is_safe = true;
        let mut is_increasing = true;
        let mut is_decreasing = true;
        for y in 1..=reports[x].len()-1 {
            let previous_level = reports[x][y-1];
            let current_level = reports[x][y];

            // INCREASE VS DECREASE
            if is_increasing && (current_level > previous_level) {
                is_decreasing = false;
            } else if is_decreasing && (previous_level > current_level) {
                is_increasing = false;
            } else {
                is_safe = false;
                break
            }

            // DIFFER 1, 2 or 3 LEVEL
            let difference = (current_level - previous_level).abs();
            if !((difference >= 1) && (difference <= 3)) {
                is_safe = false;
                break
            }
        }
        
        if is_safe {
            safe_report += 1;
        }
    }
    println!("{safe_report}")
}
