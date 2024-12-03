use std::fs::File;
use std::io::{self, BufRead};

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

fn main() {
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    read_arrays_from_file("assets/lists.txt", &mut arr1, &mut arr2);

    arr1.sort_unstable();
    arr2.sort_unstable();

    let result = array_diff_sum(&arr1, &arr2);
    println!("The sum of the elements is: {}", result);
    
}
