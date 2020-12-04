use std::fs;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("sample_input.txt")
    	.expect("Something went wrong reading the file");

    let (map_width, map_height, map) = parse_map(input);

    let mut current_down = 0;
    let mut current_right = 0;
    let mut found_trees = 0;

    let slope_down = 1;
    let slope_right = 3;

    loop {
    	current_right = current_right + slope_right;
    	current_down = current_down + slope_down;

    	println!("Step line {} col {}", current_down+1, current_right+1);

    	if map.contains(&Point {down: current_down, right: current_right}) {
    		println!("Found tree! {} {}", current_down+1, current_right+1);
    		found_trees = found_trees + 1;
    	}

    	if current_right > map_width {
	    	let wrapped_right = current_right % map_width;

	    	println!("Wrapped step line {} col {}", current_down+1, wrapped_right+1);
	    	if map.contains(&Point {down: current_down, right: wrapped_right}) {
	    		println!("Found tree! {} {}", current_down+1, wrapped_right+1);
    			found_trees = found_trees + 1;

    		}

	    }

    	if current_down > map_height {
    		break;
    	}

    }

    println!("Found trees: {}", found_trees);

  // //   let mut current_down = 1;
  // //   let mut current_right = 1;

  // //   let slope_down = 1;
  // //   let slope_right = 3;

  // //   let mut found_trees = 0;

  // //   loop {
		// // current_right = current_right + slope_right;
		// // current_down = current_down + slope_down;

		// // println!("Step {},{}", current_down, current_right);
		// // println!("{} {} % {}", current_right, map_width, (current_right%map_width));

		// // if map.contains(&Point {down: current_down, right: (current_right%map_width)}) {
		// // 	println!("found tree! {}, {}", current_down, current_right);
		// // 	found_trees = found_trees + 1;
		// // }

		
		// // if current_down >= map_height {
		// // 	break;
		// // }
  //   }

    // println!("Found trees {}", found_trees);
}

fn parse_map(input: String) -> (usize, usize, HashSet<Point>) {
	let mut map_height: usize = 0;
	let mut map_width: usize = 0;
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
    			// println!("Line: {}", line);
    			// println!("Inserting down {} right {}", down, right);
    			map.insert(Point {down, right});
    		}
    		
    	}
    }

    return (map_height, map_width, map);
}


#[derive(PartialEq, Eq, Hash)]
struct Point {
    down: usize,
    right: usize,
}