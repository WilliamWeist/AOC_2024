use std::fs::File;
use linereader::LineReader;
use regex::Regex;

fn main() {
    let myfile = "input.txt";
    let reports = file_to_2d_i32vector(myfile);

    let mut safe_report = 0;
    for x in 0..=reports.len()-1 {
        if report_is_safe(&reports[x]) {
            safe_report += 1;
        } else {
            // DAMPENER
            // TRY TO FIND IF SAFE BY REMOVING 1 ENTRY FROM THE ARRAY
            for y in 0..=reports[x].len()-1 {
                let mut _vec: Vec<i32> = Vec::new();
                for z in 0..=reports[x].len()-1 {
                    if y != z {
                        _vec.push(reports[x][z]);
                    }
                }
                if report_is_safe(&_vec) {
                    safe_report += 1;
                    break
                }
            }
        }
    }
    println!("{safe_report}")
}

fn file_to_2d_i32vector(path: &str) -> Vec<Vec<i32>> {
    let mut _vectors: Vec<Vec<i32>> = Vec::new();
    
    let file = File::open(path).expect("open");
    let mut reader = LineReader::new(file);

    let mut _vec_position = 0;
    let regex = Regex::new(r"\d+").unwrap();
    while let Some(b_line) = reader.next_line() {
        let b_line = b_line.expect("read error");
        let s_line = match std::str::from_utf8(b_line) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        _vectors.push(Vec::new());
        for match_found in regex.find_iter(s_line) {
            _vectors[_vec_position].push(s_line[match_found.start()..match_found.end()].parse::<i32>().unwrap());
        }
        _vec_position += 1;
    }
    return _vectors
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let mut is_increasing = true;
    let mut is_decreasing = true;
    for y in 1..=report.len()-1 {
        let previous_level = report[y-1];
        let current_level = report[y];

        // INCREASE VS DECREASE
        if is_increasing && (current_level > previous_level) {
            is_decreasing = false;
        } else if is_decreasing && (previous_level > current_level) {
            is_increasing = false;
        } else {
            is_safe = false;
        }

        // DIFFER 1, 2 or 3 LEVEL
        let difference = (current_level - previous_level).abs();
        if !((difference >= 1) && (difference <= 3)) {
            is_safe = false;
        }
    }
    return is_safe;
}
