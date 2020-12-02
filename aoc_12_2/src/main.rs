use std::fs;
use regex::Regex;


fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<needle>\w): (?P<passwd>\w+)").unwrap();

    // for line in input.lines() {
    	
    // 	let caps = re.captures(line).unwrap();
    // 	let min = caps["min"].parse().unwrap();
    // 	let max = caps["max"].parse().unwrap();

    	
    // 	let mat = caps["passwd"].find(&caps["needle"]);

    // 	match mat {
    // 		Some(cnt) => {
    // 			if (min..=max).contains(&cnt) {
    // 				println!("Found!");
    // 			}
    // 		}
    // 		None => {
    // 			println!("Not found!");
    // 			continue
    // 		}
    // 	}
    // }

    let count = input
    	.lines()
    	.filter(|line| {
    		let caps = re.captures(line).unwrap();
    		let min = caps["min"].parse().unwrap();
    		let max = caps["max"].parse().unwrap();
    		
    		match caps["passwd"].find(&caps["needle"]) {
    			Some(cnt) => {
    				(min..=max).contains(&cnt)
    			}
    			None => false
    		}
		}).count();

		println!("Found {} valid passwords", count);
}
   

    // let mat = "abcde".find("c");
    // println!("found match {:?}", mat);
    

    // if matches!("abcde", "c") {
    // 	print!("found match");
    // } else {
    // 	println!("no match");
    // }

    // for line in input.lines() {
    // 	println!("{:?}", line);
    // 	let caps = re.captures(line).unwrap();
    // 	println!("{:?}", caps);


    // }

   // let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
	
	// let lines = input
	// 	.lines()
	// 	.map( |line | {
	// 		println!("{}", line);
	// 	})
	// 	.collect();

//     // for line in input.lines() {
//     // 	let policy = parse_line(line);
//     // 	println!("policy.needle: {} policy.passwd: {}", policy.needle, policy.passwd);
//     // 	check(policy);
//     // 	break;
//     }
// }

// fn match_count()

// fn parse_line(line: &str) -> Policy {
// 	// println!("{:?}", line);
// 	let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
// 	let capture = re.captures(line).unwrap();
// 	// println!("{:?}", capture);
// 	let policy = Policy { 
// 		min: capture[1].parse::<i32>().unwrap(),
// 		max: capture[2].parse::<i32>().unwrap(),
// 		needle: capture[3].chars().next()?, 
// 		passwd: capture[4].to_string()
// 	};

// 	return policy;
// }
