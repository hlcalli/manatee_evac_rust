// Author: Hannah Callihan, hcallihan2020@my.fit.edu
// Author: Seth Heinzman, mheinzman2017@my.fit.edu
// Course: CSE 4250, Fall 2022
// Project: Proj2, Manatee Evacuation
// Implementation: rust
use std::io;
fn fetch_number_in_each_row() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Need number of names");

    return input.trim().parse().expect("Need a number");
}
fn fetch_row_array(text: &str, row_length: usize) -> Vec<String> {
    let mut input = String::new();
    let mut row = vec![String::new(); row_length];

    println!("{}", text);
    io::stdin().read_line(&mut input).expect("Need name");

    for i in 0..row_length {
        row[i] = input.as_str().chars().nth(i).unwrap().to_string()
    }

    return row;
}

fn main() {
    // Take input with arrays (size/age)

    let _number_in_each_row: usize = fetch_number_in_each_row();
    let _female_age_row = fetch_row_array("Please Enter Female Age Row :", _number_in_each_row);
    println!("Number in Each Row: {}", _number_in_each_row);
    println!("Female Age Row : {:?}", _female_age_row);
    // create binary search tree

    // find best match - same age, smaller male-->larger female (closest distance in size)

    // print output (make sure no '[]')
}
