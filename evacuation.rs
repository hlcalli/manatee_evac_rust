// Author: Hannah Callihan, hcallihan2020@my.fit.edu
// Author: Seth Heinzman, mheinzman2017@my.fit.edu
// Course: CSE 4250, Fall 2022
// Project: Proj2, Manatee Evacuation
// Implementation: rustc 1.57.0 (f1edd0429 2021-11-29)

use std::io;

fn fetch_number_in_each_row() -> usize {
    // Initializes type of input.
    let mut input = String::new();

    // Gets whatever the user types and shoves it in input.
    io::stdin()
        .read_line(&mut input)
        .expect("Need number of names");

    // Sets the input to a number. In this case a usize (That's what the site did).
    return input.trim().parse().expect("Need a number");
}
fn fetch_row_array(text: &str, row_length: usize) -> Vec<String> {
    // Initializes type of input.
    let mut input = String::new();

    // Establishes a vector of type String::new() and with a size of row_length.
    let mut row = vec![String::new(); row_length];

    // Prints out what you want to ask the user.
    println!("{}", text);

    // Gets whatever the user types and shoves it in input.
    io::stdin().read_line(&mut input).expect("Need name");

    // Iterates row_length times and sets the character to row[i]
    for i in 0..row_length {
        row[i] = input.as_str().chars().nth(i).unwrap().to_string()
    }

    return row;
}

fn main() {
    // Gets the number in each row.
    let _number_in_each_row: usize = fetch_number_in_each_row();

    // Fetches each user input for the rows (Female Age is the only one setup right now).
    let _female_age_row = fetch_row_array("Please Enter Female Age Row :", _number_in_each_row);

    // Prints out variables above for checking (You have to use {:?} for vectors).
    println!("Number in Each Row: {}", _number_in_each_row);
    println!("Female Age Row : {:?}", _female_age_row);



     // create manatee struct
     struct Manatee {
        size: f32,
        age: f32,
        id: f32,
    }
    
    // create binary search tree
    
    
    // find best match - same age, smaller male-->larger female (closest distance in size)
    // Use Btree
    use std::collections::BTreeSet;
use std::ops::Bound::Included;
let mut set = BTreeSet::new();
set.insert(3);
set.insert(5);
set.insert(8);
for &elem in set.range((Included(&4), Included(&8))) {
    println!("{elem}");
}
assert_eq!(Some(&5), set.range(4..).next());
    
    
    // print output (make sure no '[]')
    
    }
}
