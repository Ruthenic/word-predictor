use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("How many words would you like to make? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok().unwrap();
    line.pop();
    //println!("{}", line);
    let mut num_of_chars = line.parse::<i32>().ok().unwrap();
    num_of_chars = num_of_chars - 1;
    println!("What word would you like to start with? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line.pop();
    let prev_char = line;
    //println!("{}", prev_char);
    let mut index = 0;
	let mut choices: Vec<String> = vec![];
	let mut weights: Vec<i32> = vec![];
    let mut f = File::open("counts.txt").ok().unwrap();
    let mut file_string = String::new();
    f.read_to_string(&mut file_string);
    let values = file_string;
	let mut file = std::fs::File::create("out.txt").expect("create failed");
	loop {
		println!("{}", prev_char);
		file.write(prev_char.as_bytes());
		file.write(" ".as_bytes());
		if index == num_of_chars {
			break;
		}
    	choices = vec![];
    	let mut weights = vec![];
    	let prev_char = format!("{};", &prev_char.replace(";", ""));
	    for pair in values.lines() {
	    	pair.to_string();
	    	//println!("{}\n{}", pair, prev_char);
	    	if pair.starts_with(&prev_char) {
	    		choices.push(format!("{};{}", pair.split(";").collect::<Vec<&str>>()[0], pair.split(";").collect::<Vec<&str>>()[1]));
				weights.push(pair.split(";").collect::<Vec<&str>>()[2].parse::<i32>().ok().unwrap());
	    	}
	    }
	    let dist = WeightedIndex::new(&weights).unwrap();
	    let mut rng = thread_rng();
	    let prev_char: &str = &choices[dist.sample(&mut rng)].split(";").collect::<Vec<&str>>()[1];
	    println!("{}", prev_char);
	    index+=1;
	}
}
