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

fn quick_sort(arr: &mut [i32], begin: usize, end: usize) {
    if begin < end {
        let partition_index = partition(arr, begin, end);

        if partition_index > 0 {
            quick_sort(arr, begin, partition_index - 1);
        }
        quick_sort(arr, partition_index + 1, end);
    }
}

fn partition(arr: &mut [i32], begin: usize, end: usize) -> usize {
    let pivot = arr[end];
    let mut i = begin as isize - 1;

    for j in begin..end {
        if arr[j] <= pivot {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    arr.swap((i + 1) as usize, end);
    (i + 1) as usize
}

fn array_diff_sum(arr1: &mut [i32], arr2: &mut [i32]) -> i32 {
    arr1.iter()
        .zip(arr2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn main() {
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    read_arrays_from_file("assets/lists.txt", &mut arr1, &mut arr2);

    let len1 = arr1.len();
    let len2 = arr2.len();

    if len1 != len2 {
        panic!("The arrays must have the same length");
    }

    if !arr1.is_empty() {
        quick_sort(&mut arr1, 0, len1 - 1);
        quick_sort(&mut arr2, 0, len2 - 1);

        let result = array_diff_sum(&mut arr1, &mut arr2);
        println!("The sum of the elements is: {}", result);
    }
}
