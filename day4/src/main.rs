use std::fs;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");


    	let mut passport_fields: Vec<&str> = Vec::new();
    	let mut valid_passports = 0;

    	for line in input.lines() {
    		println!("{}", line);

    		// blank line
    		if line.is_empty() {
    			if passport_fields.len() >= 8 {
    				valid_passports +=1;
    			} else if passport_fields.len() >= 7 {
    				if ! passport_fields.contains(&"cid") {
    					valid_passports += 1;
    				}
    			}
    			println!("{}", valid_passports);

    			passport_fields = Vec::new();
    			continue;
    		}

    		for field in line.split_ascii_whitespace() {
    			let field: Vec<&str> = field.split(':').collect();
    			passport_fields.push(field[0]);
    		}

    		
    	}

    	//println!("valid_passports {}", valid_passports);
}
