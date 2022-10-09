// Author: Hannah Callihan, hcallihan2020@my.fit.edu
// Author: Seth Heinzman, mheinzman2017@my.fit.edu
// Course: CSE 4250, Fall 2022
// Project: Proj2, Manatee Evacuation
// Implementation: rust


fn main() {
    // take input with arrays (size/age)
    use io::BufRead;
    use std::io; // necessary to have `.lines()` on
                 // locked stdin available
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        let numberofnames: usize = lines
            .next()
            .expect("Need input from stdin") // checking for any line being present
            .expect("Need number of names")
            .trim()
            .parse()
            .expect("Need a number");
        let names = lines
            .take(numberofnames)
            .map(|line| line.expect("Need name").trim().into())
            .collect::<Vec<String>>();
        if names.len() < numberofnames {
            panic!("Need more names, only got {}!", names.len());
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



