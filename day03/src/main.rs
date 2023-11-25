use std::fs;
use itertools::Itertools;

fn read_input_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

// translation
// > == (1, 0)
// v == (0, -1)
// < == (-1, 0)
// ^ == (0, 1)

fn main() {
    let file_contents = read_input_file("input.txt");
    let chars: Vec<char> = file_contents.chars().collect();

    let mut house_visits: Vec<Vec<i32>> = Vec::new();
    let start_coordinate = vec![0, 0];
    house_visits.push(start_coordinate);

    for x in chars {
        let old_coord = house_visits.last().unwrap();
        if x == '>' {
            let dist_to_move = vec![1, 0];
            let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
            house_visits.push(new_coord);
        }
        else if x == 'v' {
            let dist_to_move = vec![0, -1];
            let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
            house_visits.push(new_coord);
        }
        else if x == '<' {
            let dist_to_move = vec![-1, 0];
            let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
            house_visits.push(new_coord);
        }
        else if x == '^' {
            let dist_to_move = vec![0, 1];
            let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
            house_visits.push(new_coord);
        }
    }
    let houses_visited: Vec<_> = house_visits.into_iter().unique().collect();
    println!("{}", houses_visited.len());
}
