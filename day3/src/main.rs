use std::fs;

fn main() {
    
    let mut right: usize = 0;
    let mut tree_count = 0;
    let slope: usize = 3;

    let input = fs::read_to_string("sample_input.txt")
    	.expect("Something went wrong reading the file");

    
    let mut lines = input.lines();

    // consume first line
    let size = lines.next().unwrap().chars().count();
    right = right + slope;

    for line in lines {
    	
    	if line.chars().nth(right).unwrap() == '#' {
    		tree_count += 1;
    	}

    	right = (right + slope) % size;
    }

    println!("Trees: {}", tree_count);
 }