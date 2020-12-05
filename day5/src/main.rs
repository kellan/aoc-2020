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

    println!("largest_seat_id {}", largest_seat_id);

    seat_ids.sort();

    for i in 1..seat_ids.len() {
        //println!("{} {} {}", i, seat_ids[i], seat_ids[i-1]);
        if seat_ids[i] != seat_ids[i-1]+1 {
            println!("part2 {}", seat_ids[i]-1);
        }
    }    
}

fn seat_id(seat_rule: &str) -> usize {
    let bin_seat = seat_rule.replace("F", "0").replace("B", "1")
        .replace("L", "0").replace("R", "1");

    return usize::from_str_radix(&bin_seat, 2).unwrap();
}