// Author: Hannah Callihan, hcallihan2020@my.fit.edu
// Author: Seth Heinzman, sheinzman@my.fit.edu
// Course: CSE 4250, Fall 2022
// Project: Proj2, Manatee Evacuation
// Implementation: rust



fn main() {
// take input with arrays (size/age)
use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let _=stdout().flush();
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

// create manatee struct
struct Manatee {
    size: f32,
    age: f32,
    id: f32,
}

// create binary search tree


// find best match - same age, smaller male-->larger female (closest distance in size)


// print output (make sure no '[]')

}