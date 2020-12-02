use std::fs;
use regex::Regex;


fn main() {

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

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

    	println!("valid passwds: {}", valid);
}