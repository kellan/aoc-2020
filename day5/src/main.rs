use std::fs;


fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("input.txt")
    	.expect("Something went wrong reading the file");

    let mut largest_seat_id = 0;

    let mut seat_ids: Vec<usize> = Vec::new();

    for line in input.lines() {
    	let seat_id = seat_id(line);
    	seat_ids.push(seat_id);

    	if seat_id > largest_seat_id {
    		largest_seat_id = seat_id
    	}
    }

	seat_ids.sort();

	for (pos, e) in seat_ids.iter().enumerate() {
		if seat_ids[pos+1] - e > 1 {
			println!("{}", e+1);
			break;
		}
	}
    
}

fn seat_id(seat_rule: &str) -> usize {
	let mut row = [0, 127];

    for c in seat_rule[0..7].chars() {
    	let diff = (row[1] - row[0])/2 as usize;
    	
    	if c == 'F' {
    		row = [row[0], row[0]+diff];
    	} else if c == 'B' {
    		row = [row[0]+diff+1, row[1]];
    	}
    }

    let mut col = [0, 7];

    for c in seat_rule[7..10].chars() {
    	let diff = (col[1] - col[0])/2 as usize;

    	if c == 'L' {
    		col = [col[0], col[0]+diff];
    	} else if c == 'R' {
    		col = [col[0]+diff+1, col[1]];
    	}
    }

    let seat_id = row[0]*8 + col[0];

    //println!("{:?}, {:?} = {}", row, col, seat_id);

    return seat_id;
}