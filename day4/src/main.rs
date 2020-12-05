use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    let valid_passports = part1(&input);

    println!("valid_passports {}", valid_passports);

    let valid_passports = part2(&input);

    println!("valid_passports2 {}", valid_passports);
}

fn part2(input: &String) -> usize {

    let mut valid_passports: usize = 0;
	let mut lines = input.lines();
    let mut valid_parts: usize = 0;

	loop {
		let line = lines.next();

		if line.is_some() && !line.unwrap().is_empty() {
			
            for field in line.unwrap().split_ascii_whitespace() {
    			let field: Vec<&str> = field.split(':').collect();
    			
                if valid_field(&field) {
    				valid_parts += 1;
    			}

    		}

            continue;

		}

		// if we got here it's because we've reached the end of a passport

    	if valid_parts == 7 {
    		valid_passports += 1;
    	}

        valid_parts = 0;

		if line.is_none() {
    		break;
    	}	
	}

    return valid_passports;
}

fn valid_field(field: &Vec<&str>) -> bool {
	let valid: bool = match field[0] {
		"byr" => {
			let year = field[1].parse::<i32>().unwrap();
			if year >= 1920 && year <= 2002 { true } else { false }
		},
		"iyr" => {
			let year = field[1].parse::<i32>().unwrap();
			if year >= 2010 && year <= 2020 { true } else { false }
		},
		"eyr" => {
			let year = field[1].parse::<i32>().unwrap();
			if year >= 2020 && year <= 2030 { true } else { false }
		},
		"hgt" => {
            let value = field[1];
            let unit = &value[(value.len()-2)..value.len()];
            
			//let unit: String = field[1].chars().rev().take(2).collect();
            
            match unit {
                "cm" => {
                    let number = value[0..value.len()-2].parse::<i32>().unwrap();
                    number >= 150 && number <= 193
                },
                "in" => {
                    let number = value[0..value.len()-2].parse::<i32>().unwrap();
                    number >= 59 && number <= 76 
                },
                _ => false,
            }
		},
        "hcl" => {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(field[1])
        },
        "ecl" => {
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&field[1])
        },
        "pid" => {
            field[1].len() == 9 && field[1].parse::<usize>().is_ok()
        },
		_ => false,
	};

	return valid;
}

fn part1(input: &String) -> usize {
	let mut passport_fields: Vec<&str> = Vec::new();
    let mut valid_passports = 0;
	let mut lines = input.lines();

    loop {
    	let line = lines.next();

    	if line.is_some() && !line.unwrap().is_empty() {
    			
    		for field in line.unwrap().split_ascii_whitespace() {
    			let field: Vec<&str> = field.split(':').collect();
    			passport_fields.push(field[0]);
    		}

    		continue;
    	}

    	// if we got here it's because we've reached the end of a passport
    		
    	if valid_passport(passport_fields) {
    		valid_passports += 1;
    	}

    	passport_fields = Vec::new();

    	if line.is_none() {
    		break;
    	}
    }

    return valid_passports;
}

fn valid_passport(passport: Vec<&str>) -> bool {
	if passport.len() >= 8 {
    	return true
    
    } else if passport.len() >= 7 {
    	if ! passport.contains(&"cid") {
    		return true;
    	}
    }

    return false;
}
