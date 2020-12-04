use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Trees part1: {} part2: {}", part1, part2);
 }

 fn part1(input: &String) -> usize {

    let tree_count = toboggan(input, 3, 1);
    return tree_count;
 }

 fn part2(input: &String) -> usize {
 	let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
 	//let mut counts: Vec<usize> = [];

 	let mut count = 1;

 	for slope in slopes.iter() {
	 	let found = toboggan(input, slope.0, slope.1);
	 	count = found * count;
	}

    return count;	
 }

 // fn part2() -> usize {
 // 	let input = fs::read_to_string("input.txt")
 //    	.expect("Something went wrong reading the file");

 // 	let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    
 //    for slope in slopes.iter() {
 //    	let tree_count = toboggan(input, 1, 2);
 //    }

 //    return 0
 // }

fn toboggan(input: &String, slope_right: usize, slope_down: usize) -> usize {

	let mut right: usize = 0;
    let mut tree_count = 0;
    
    let mut lines = input.lines().step_by(slope_down);

    // consume first line
    let size = lines.next().unwrap().chars().count();
    right = right + slope_right;

    for line in lines {
    	
    	if line.chars().nth(right).unwrap() == '#' {
    		tree_count += 1;
    	}

    	right = (right + slope_right) % size;
    }

    return tree_count;
}