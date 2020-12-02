use std::fs;


fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    // println!("With text:\n{}", input);

    let numbers = input
    	.lines()
    	.map( |line| {
    		line.parse::<i32>().unwrap()
    	})
    	.collect::<Vec<i32>>();

    println!("{:?}", numbers);
    	
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn test_add() {
	// 	assert_eq!(add(1,2), 3);
	// }
}