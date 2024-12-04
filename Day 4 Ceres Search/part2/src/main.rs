use std::fs::File;
use linereader::LineReader;

static MAS: [char; 3] = ['M', 'A', 'S'];

fn main() {
    let myfile: &str = "input.txt";
    let input: Vec<Vec<char>> = file_to_2d_char_vector(&myfile);
    let result = count_x_mas(input);
    println!("{result}");
}

fn count_x_mas(input: Vec<Vec<char>>) -> i32 {
    let mut result: i32 = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == MAS[1] {
                if x >= 1 && x <= input.len()-2 && y >= 1 && y <= input[x].len()-2 {
                    // M . M
                    // . A .
                    // S . S
                    if input[x-1][y-1] == MAS[0] && input[x-1][y+1] == MAS[0] && 
                       input[x+1][y-1] == MAS[2] && input[x+1][y+1] == MAS[2] {
                        result += 1;
                    }
                    // S . M
                    // . A .
                    // S . M
                    if input[x-1][y-1] == MAS[2] && input[x-1][y+1] == MAS[0] && 
                       input[x+1][y-1] == MAS[2] && input[x+1][y+1] == MAS[0] {
                        result += 1;
                    }
                    // S . S
                    // . A .
                    // M . M
                    if input[x-1][y-1] == MAS[2] && input[x-1][y+1] == MAS[2] && 
                       input[x+1][y-1] == MAS[0] && input[x+1][y+1] == MAS[0] {
                        result += 1;
                    }
                    // M . S
                    // . A .
                    // M . S
                    if input[x-1][y-1] == MAS[0] && input[x-1][y+1] == MAS[2] && 
                       input[x+1][y-1] == MAS[0] && input[x+1][y+1] == MAS[2] {
                        result += 1;
                    }
                }
            }
        }
    }
    return result
}

fn file_to_2d_char_vector(path: &str) -> Vec<Vec<char>> {
    let mut vector: Vec<Vec<char>> = Vec::new();
    let file = File::open(path).expect("open");
    let mut reader = LineReader::new(file);
    while let Some(b_line) = reader.next_line() {
        let b_line = b_line.expect("read error");
        let s_line = match std::str::from_utf8(b_line) {
            Ok(v) => v.trim(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        vector.push(s_line.chars().collect());
    }
    return vector
}
