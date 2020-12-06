use std::fs;
use std::collections::HashSet;


fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("yes {}", part1(&input));

    println!("all_yes {}", part2(&input));

}

fn part2(input: &String) -> usize {

	let mut cnt: usize = 0;

	for group in input.split("\n\n") {
		//println!("{}", group);
		let mut sets = group.lines()
			.map(|line| {
				line.chars().collect::<HashSet<_>>()
			});

		let inter = sets
			.next() // need first element for fold
			.map(|set| {
				sets.fold(set, |set1, set2| {
					set1.intersection(&set2).cloned().collect()
				})
			}).unwrap();
		
		cnt = cnt+inter.len();
	}

	return cnt
    
}
fn part1(input: &String) -> usize {
	let questions: usize = input.split("\n\n")
    	.map( |group| {
    		let set = group
    			.lines()
    			.flat_map(|a| a.chars())
    			.collect::<HashSet<_>>();
    		set.len()
    	})
    	.sum();

    return questions;
}
