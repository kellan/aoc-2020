use std::fs;


fn main() {
    let target = 2020;

    // println!("Hello, world!");
    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    // println!("With text:\n{}", input);

    let mut numbers = input
    	.lines()
    	.map( |line| {
    		line.parse::<i32>().unwrap()
    	})
    	.collect::<Vec<i32>>();

    //println!("{:?}", numbers);

    numbers.sort_unstable();

    for a in &numbers {
        for b in &numbers {
            if a+b == target {
                println!("{} + {} = {}", a, b, a+b);
                println!("{} * {} = {}", a, b, a*b);
                break;
            }
        }
    }
    	
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn test_add() {
	// 	assert_eq!(add(1,2), 3);
	// }
}