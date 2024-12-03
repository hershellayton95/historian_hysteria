use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_arrays_from_file(filename: &str, arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) {
    let file = File::open(filename).expect("Error opening the file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Error reading the line");
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Error converting the number"))
            .collect();

        if values.len() == 2 {
            arr1.push(values[0]);
            arr2.push(values[1]);
        }
    }
}

fn array_diff_sum(arr1: &[i32], arr2: &[i32]) -> i32 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn array_occurrences_sum (arr1: &[i32], arr2: &[i32]) -> i32 {

    let mut frequency_map = HashMap::new();
    for &num in arr2 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    arr1.iter()
    .map(|&num| num * frequency_map.get(&num).cloned().unwrap_or(0))
    .sum()
}

fn main() {
    let mut left_column = Vec::new();
    let mut right_column = Vec::new();

    read_arrays_from_file("assets/lists.txt", &mut left_column, &mut right_column);

    left_column.sort_unstable();
    right_column.sort_unstable();

    let result1 = array_diff_sum(&left_column, &right_column);

    println!("Part 1: {}", result1);

    let result2 = array_occurrences_sum(&left_column, &right_column);
    println!("Part 2: {}", result2);
    
}
