use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;
use std::iter::FromIterator;

fn read_input_data() -> io::Result<Vec<i32>>{
    let f = File::open("inputData.txt")?;
    let f = BufReader::new(f);

    let mut vec = Vec::new();

    for line in f.lines() {
        vec.push(line.unwrap().parse::<i32>().unwrap());
    }

    Ok(vec)
}

fn loop_with_fixed_number(fixed: i32, numbers: &[i32]) -> i32{
    for &n in numbers {
        if fixed+n==2020 {
            return n;
        }
    }
    -1
}

fn outer_loop(numbers: &[i32]) -> (i32, i32) {
    let mut res:i32;
    for &n in numbers {
        res = loop_with_fixed_number(n, &numbers);
        if res != -1 {
            return (n, res)
        }
    }
    (-1,-1)
}

fn simple_solution() -> i32 {
    let numbers = read_input_data().unwrap();
    let (n,m) = outer_loop(&numbers);
    return n*m
}

fn find_pair_from_sorted_array(numbers: &[i32]) -> (i32, i32) {
    let mut sorted_numbers: Vec<i32> = numbers.to_vec();
    sorted_numbers.sort_unstable();
    let goal:i32 = 2020;
    let mut low_index:usize = 0;
    let mut high_index:usize = sorted_numbers.len()-1;
    let mut n = sorted_numbers[low_index];
    let mut m = sorted_numbers[high_index];
    while n+m != goal {
        if n+m > goal {
            high_index -= 1
        } else {
            low_index += 1
        }
        n = sorted_numbers[low_index];
        m = sorted_numbers[high_index];
    }
    (n,m)
}

fn mediate_solution() -> i32{
    let numbers = read_input_data().unwrap();
    let (n,m) = find_pair_from_sorted_array(&numbers);
    return n*m
}

fn find_numbers_from_set(numbers: Vec<i32>) -> (i32, i32) {
    let numbers_set:HashSet<i32> = HashSet::from_iter(numbers);
    let goal = 2020;
    for &n in &numbers_set {
        if numbers_set.contains(&(goal-n)) {
            return (n, goal-n);
        }
    }
    (-1, -1)
}

fn fast_solution() -> i32 {
    let numbers = read_input_data().unwrap();
    let (n,m) = find_numbers_from_set(numbers);
    return n*m
}

fn main() {
    let product = simple_solution();
    println!("{}", product);
    let product = mediate_solution();
    println!("{}", product);
    let product = fast_solution();
    println!("{}", product);
    let start = Instant::now();
    for _ in 1..10000 {
        simple_solution();
    }
    println!("{}", start.elapsed().as_millis());
    let start = Instant::now();
    for _ in 1..10000 {
        mediate_solution();
    }
    println!("{}", start.elapsed().as_millis());
    let start = Instant::now();
    for _ in 1..10000 {
        fast_solution();
    }
    println!("{}", start.elapsed().as_millis());
}
