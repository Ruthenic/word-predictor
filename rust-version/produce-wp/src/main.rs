use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("How many words would you like to make? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    //println!("{}", line);
    let num_of_chars = line.parse::<i32>()-1;
    println!("What word would you like to start with? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    let prev_char = line;
    //println!("{}", prev_char);
    let mut index = 0;
	let mut choices = vec![];
	let mut weights = vec![];
    let mut f = File::open("counts.txt");
    let mut file_string = String::new();
    f.read_to_string(&mut file_string);
    let values = file_string.split("\n");
	loop {
		if index == num_of_chars {
			break;
		}
    	choices = vec![];
    	weights = vec![];
	    for pair in values {
	    	pair.to_string();
	    	if pair.starts_with(prev_char) {
	    		//possibles.push(pair);
	    	}
	    }
	    index+=1;
	}
}
