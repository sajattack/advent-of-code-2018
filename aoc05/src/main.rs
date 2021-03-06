extern crate rayon;

#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::str;

use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;

fn main() {
    let input: String = get_input(5).expect("Failed to retrieve input");
    part1(input.clone());
    part2(input.clone());
}

fn part1(mut input: String) {
    input = input.trim().to_string();
    let input = reduce(input);    
    let answer = input.len();
    println!("{}", answer);
    println!("{}", submit_answer(5, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(mut input: String) { 
    input = input.trim().to_string();
    let answer = (b'A'..b'Z'+1)
    .into_par_iter()
    .map(|letter| {
        let mut working_copy = input.clone();
        working_copy = working_copy.replace(str::from_utf8(&[letter]).unwrap(), "");
        working_copy = working_copy.replace(str::from_utf8(&[letter+32]).unwrap(), "");
        working_copy = reduce(working_copy);
        working_copy.len()
    }).min().unwrap();
    println!("{}", answer);
    println!("{}", submit_answer(5, 2, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn reduce(mut working_copy: String) -> String {
    loop {
        let prev_input_len = working_copy.len();
        for combo in (b'a'..=b'z').zip(b'A'..=b'Z') {
            working_copy = working_copy.replace(
                str::from_utf8(&[combo.0, combo.1]).unwrap(), "");
            working_copy = working_copy.replace(
                str::from_utf8(&[combo.1, combo.0]).unwrap(), "");
        }
        if working_copy.len() == prev_input_len { break; }
    }
    return working_copy;
}
