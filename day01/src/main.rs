use std::fs;
use std::str;

fn read_input_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

fn main() {
    let file_contents: String = read_input_file("input.txt");
    
    let mut open_count: i32 = 0;

    let mut closed_count: i32 = 0;

    let mut iteration_count: i32 = 0;

    for x in file_contents.chars() {
        iteration_count +=1;
        println!("Iteration: {}", iteration_count);
        if x == '(' {
            open_count += 1;
        } else if x == ')' {
            closed_count += 1;
        }
        let diff: i32 = open_count - closed_count;
        if diff < 0 {
            break;
        }
    }
    // println!("Open Count: {}", open_count);
    // println!("Closed Count: {}", closed_count);
    // let diff: i32 = open_count - closed_count;
    // println!("Diff: {}", diff);
}
