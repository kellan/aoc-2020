use std::fs;
use regex::Regex;


fn main() {

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

   	//let valid = policy1(input);

   	let valid = policy2(input);

	println!("policy1 valid passwds: {}", valid);
}

fn policy1(input: String) -> usize {
	let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<needle>\w): (?P<passwd>\w+)").unwrap();

    let valid = input
    	.lines()
    	.filter(|line| {
    		//println!("{:?}", line);
    		let caps = re.captures(line).unwrap();
    		let min = caps["min"].parse().unwrap();
    		let max = caps["max"].parse().unwrap();
    		
    		let cnt = caps["passwd"].matches(&caps["needle"]).count();
    		
    		(min..=max).contains(&cnt)
    	})
    	.count();

    return valid
}

fn policy2(input: String) -> usize {
	let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<needle>\w): (?P<passwd>\w+)").unwrap();

	let valid = input
    	.lines()
    	.filter(|line| {
    		//println!("{}", line);

    		let caps = re.captures(line).unwrap();
    		
    		let pos1: usize = caps["min"].parse().unwrap();
    		let pos2: usize = caps["max"].parse().unwrap();

    		let needle = caps["needle"].chars().next().unwrap();

    		let passwd: Vec<char> = caps["passwd"].chars().collect();

    		// xor

    		if (passwd[pos1-1] == needle) ^ (passwd[pos2-1] == needle) {
    		//	println!("valid: {}", line);
    			true
    		} else {
    		//	println!("invalid: {}", line);
    			false
    		}
    	})
    	.count();

    return valid;
}