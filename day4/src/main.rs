use std::fs;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");


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
    		
    		if passport_fields.len() >= 8 {
    			valid_passports +=1;
    		} else if passport_fields.len() >= 7 {
    			if ! passport_fields.contains(&"cid") {
    				valid_passports += 1;
    			}
    		}

    		passport_fields = Vec::new();

    		if line.is_none() {
    			break;
    		}
    	}

    	println!("valid_passports {}", valid_passports);
}
