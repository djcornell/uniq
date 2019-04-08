use std::fs;
use std::env;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    
	let mut frequency: HashMap<String, u32> = HashMap::new();

	let input = env::args().nth(1);
    let reader: Box<BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
    };

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines().map(|l| l.unwrap() ) {
		
	    let counter = frequency.entry(line.clone()).or_insert(0);
		*counter += 1;

		if *counter == 1 {
			println!("{}", line)
		}

	}

}
