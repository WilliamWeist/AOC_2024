use std::fs::File;
use linereader::LineReader;

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn main() {
    let myfile: &str = "input.txt";
    let input: Vec<Vec<char>> = file_to_2d_char_vector(&myfile);
    let result = count_xmas(input);
    println!("{result}");
}

fn count_xmas(input: Vec<Vec<char>>) -> i32 {
    let xmas_len = XMAS.len();
    let mut result: i32 = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == XMAS[0] {
                // NORTH SEARCH
                if x >= xmas_len-1 {
                    if input[x-1][y] == XMAS[1] && input[x-2][y] == XMAS[2] && input[x-3][y] == XMAS[3] {
                        result += 1;
                    }
                }
                // NORTH WEST SEARCH
                if x >= xmas_len-1 && y >= xmas_len-1 {
                    if input[x-1][y-1] == XMAS[1] && input[x-2][y-2] == XMAS[2] && input[x-3][y-3] == XMAS[3] {
                        result += 1;
                    }
                }
                // WEST SEARCH
                if y >= xmas_len-1 {
                    if input[x][y-1] == XMAS[1] && input[x][y-2] == XMAS[2] && input[x][y-3] == XMAS[3] {
                        result += 1;
                    }
                }
                // SOUTH WEST SEARCH
                if x+xmas_len <= input.len() && y >= xmas_len-1 {
                    if input[x+1][y-1] == XMAS[1] && input[x+2][y-2] == XMAS[2] && input[x+3][y-3] == XMAS[3] {
                        result += 1;
                    }
                }
                // SOUTH SEARCH
                if x+xmas_len <= input.len() {
                    if input[x+1][y] == XMAS[1] && input[x+2][y] == XMAS[2] && input[x+3][y] == XMAS[3] {
                        result += 1;
                    }
                }
                // SOUTH EAST SEARCH
                if x+xmas_len <= input.len() && y+xmas_len <= input[x].len() {
                    if input[x+1][y+1] == XMAS[1] && input[x+2][y+2] == XMAS[2] && input[x+3][y+3] == XMAS[3] {
                        result += 1;
                    }
                }
                // EAST SEARCH
                if y+xmas_len <= input[x].len() {
                    if input[x][y+1] == XMAS[1] && input[x][y+2] == XMAS[2] && input[x][y+3] == XMAS[3] {
                        result += 1;
                    }
                }
                // NORTH EAST SEARCH
                if x >= xmas_len-1 && y+xmas_len <= input[x].len() {
                    if input[x-1][y+1] == XMAS[1] && input[x-2][y+2] == XMAS[2] && input[x-3][y+3] == XMAS[3] {
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
