use std::fs;
use regex::Regex;

struct Policy {
	min: i32,
	max: i32,
	needle: String,
	passwd: String,
}

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    for line in input.lines() {
    	let policy = parse_line(line);
    	println!("policy.needle: {} policy.passwd: {}", policy.needle, policy.passwd);
    	check(policy);
    	break;
    }
}

fn check(policy: Policy) -> Option<usize> {
	let c = policy.passwd.as_str().matches(policy.needle.as_str()).count();
	println!("c: {:?}", c);
	// //println!("c: {}", c);
	// if c < policy.min or c > policy.max {
	// 	return Some(c);
	// } else {
	// 	return None;
	// }

	if c > policy.min.try_into().unwrap() {
		return Some(c)
	} else {
		return None
	}
}

fn parse_line(line: &str) -> Policy {
	// println!("{:?}", line);
	let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
	let capture = re.captures(line).unwrap();
	// println!("{:?}", capture);
	let policy = Policy { 
		min: capture[1].parse::<i32>().unwrap(),
		max: capture[2].parse::<i32>().unwrap(),
		needle: capture[3].to_string(), 
		passwd: capture[4].to_string()
	};

	return policy;
}
