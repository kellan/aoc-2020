use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("sample_input.txt")
    	.expect("Something went wrong reading the file");

    let mut map_width = 0;
    let mut map_height = 0;

    let mut map: HashSet<Point> = HashSet::new();

    for (down,line) in input.lines().enumerate() {
    	if down > map_height {
    		map_height = down;
    	}

    	for (right, c) in line.chars().enumerate() {
    		if right > map_width {
    			map_width = right;
    		}

    		if c == '#' {
    			println!("found tree: {},{} {}", down,right, c);
    			map.insert(Point {x: right, y: down});
    		}
    		
    	}
    }

    let mut current_x = 0;
    let mut current_y = 0;

    let slope_x = 3;
    let slope_y = 1;

    let mut found_trees = 0;

    loop {
		current_x = (current_x + slope_x) % map_width;
		current_y = current_y + slope_y;

		if map.contains(&Point {x: current_x, y: current_y}) {
			//println!("found tree! {}, {}", current_x, current_y);
			found_trees = found_trees + 1;
		}

		if current_y >= map_height {
			break;
		}
    }

    println!("Found trees {}", found_trees);
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}