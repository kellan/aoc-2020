use std::fs;
use std::collections::HashSet;

fn main() {
    let target = 2020;

    // println!("Hello, world!");
    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    
    let numbers: HashSet<i32> = input
        .lines()
        .map(|line| {
            line.parse::<i32>().unwrap()
        })
        .collect();

    match pair(numbers, target) {
        Some((a,b)) => {
            println!("{} * {} = {}", a, b, a*b);
        }
        None => println!("no match"),
    }
//    triple(numbers, target);

}
    	
fn pair(numbers: HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    for n in &numbers {
        let query = target-n;
        if numbers.contains(&query) {
            return Some((*n, target-n));
        }
    }

    None
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn test_add() {
	// 	assert_eq!(add(1,2), 3);
	// }
}