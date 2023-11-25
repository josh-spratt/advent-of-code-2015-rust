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

    let mut santa_house_visits: Vec<Vec<i32>> = Vec::new();
    let mut robo_santa_house_visits: Vec<Vec<i32>> = Vec::new();
    let santa_start_coordinate = vec![0, 0];
    let robo_santa_start_coordinate = vec![0, 0];
    santa_house_visits.push(santa_start_coordinate);
    robo_santa_house_visits.push(robo_santa_start_coordinate);

    let mut iteration_number: i32 = 0;

    for x in chars {
        iteration_number += 1;
        if iteration_number % 2 == 0 {
            let old_coord = robo_santa_house_visits.last().unwrap();
            if x == '>' {
                let dist_to_move = vec![1, 0];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                robo_santa_house_visits.push(new_coord);
            }
            else if x == 'v' {
                let dist_to_move = vec![0, -1];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                robo_santa_house_visits.push(new_coord);
            }
            else if x == '<' {
                let dist_to_move = vec![-1, 0];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                robo_santa_house_visits.push(new_coord);
            }
            else if x == '^' {
                let dist_to_move = vec![0, 1];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                robo_santa_house_visits.push(new_coord);
            }
        }
        else {
            let old_coord = santa_house_visits.last().unwrap();
            if x == '>' {
                let dist_to_move = vec![1, 0];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                santa_house_visits.push(new_coord);
            }
            else if x == 'v' {
                let dist_to_move = vec![0, -1];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                santa_house_visits.push(new_coord);
            }
            else if x == '<' {
                let dist_to_move = vec![-1, 0];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                santa_house_visits.push(new_coord);
            }
            else if x == '^' {
                let dist_to_move = vec![0, 1];
                let new_coord = vec![&old_coord[0] + &dist_to_move[0], &old_coord[1] + &dist_to_move[1]];
                santa_house_visits.push(new_coord);
            }
        }
    }
    
    let santa_houses_visited: Vec<_> = santa_house_visits.into_iter().unique().collect();
    let robo_santa_houses_visited: Vec<_> = robo_santa_house_visits.into_iter().unique().collect();

    let mut all_houses_visited = santa_houses_visited;
    all_houses_visited.extend(robo_santa_houses_visited);
    let all_houses_visited: Vec<_> = all_houses_visited.into_iter().unique().collect();

    println!("{:?}", all_houses_visited.len());
}
