use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_lines_from_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_path).expect("File does not exist at path");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calc_sa(dimensions: &Vec<i32>) -> i32 {
    let sa1: i32 = dimensions[0] * dimensions[1];
    let sa2: i32 = dimensions[1] * dimensions[2];
    let sa3: i32 = dimensions[0] * dimensions[2];

    let sa_vec: Vec<i32> = vec![sa1, sa2, sa3];

    let min_sa: Option<&i32> = sa_vec.iter().min();

    let subtotal_sa: i32 = (2 * &sa_vec[0]) + (2 * &sa_vec[1]) + (2 * &sa_vec[2]);

    let total_sa: i32 = subtotal_sa + min_sa.unwrap_or(&0);

    return total_sa;
}

fn calc_perimeter_with_volume(dimensions: &Vec<i32>) -> i32 {
    let perim1: i32 = 2 * (dimensions[0] + dimensions[1]);
    let perim2: i32 = 2 * (dimensions[1] + dimensions[2]);
    let perim3: i32 = 2 * (dimensions[0] + dimensions[2]);

    let volume: i32 = dimensions[0] * dimensions[1] * dimensions[2];

    let perim_vec: Vec<i32> = vec![perim1, perim2, perim3];

    let min_perim: Option<&i32> = perim_vec.iter().min();

    let ribbon_length: i32 = min_perim.unwrap_or(&0) + volume;

    return ribbon_length;
}

// v = lwh
// perimeter of smallest side?

fn main() {
    let lines: Vec<String> = read_lines_from_file("input.txt");

    let mut parsed_lines: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let i32_lines: Vec<i32> = line
            .split("x")
            .map(|x: &str| x.parse::<i32>().unwrap())
            .collect();
        parsed_lines.push(i32_lines);
    }

    let mut cumulative_sa: Vec<i32> = Vec::new();

    for line in &parsed_lines {
        let sa = calc_sa(line);
        cumulative_sa.push(sa);
    }

    let sa: i32 = cumulative_sa.iter().sum();

    println!("{}", sa);

    let mut ribbon_lengths: Vec<i32> = Vec::new();

    for line in &parsed_lines {
        let ribbon_required = calc_perimeter_with_volume(line);
        ribbon_lengths.push(ribbon_required);
    }

    let ribbon_length: i32 = ribbon_lengths.iter().sum();

    println!("{}", ribbon_length);
}
