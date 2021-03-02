use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::distributions::Uniform;
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
    let mut prev_char = line;
    //println!("{}", prev_char);
    let mut index = 0;
	let mut choices: Vec<String> = vec![];
	let mut weights: Vec<i32> = vec![];
    let mut f = File::open("counts.txt").ok().unwrap();
    //let mut f = File::open("counts.txt").expect("unable to find counts.txt");
    let mut file_string = String::new();
    f.read_to_string(&mut file_string);
    let values = file_string;
	let mut file = std::fs::File::create("out.txt").expect("create failed");
	loop {
		println!("{}", prev_char);
		file.write(prev_char.replace("\\n", "\n").as_bytes());
		file.write(" ".as_bytes());
		if index == num_of_chars {
			break;
		}
    	choices = vec![];
    	let mut weights = vec![];
    	prev_char = format!("{};", &prev_char.replace(";", ""));
	    for pair in values.lines() {
	    	pair.to_string();
	    	//println!("{}\n{}", pair, prev_char);
	    	if pair.starts_with(&prev_char) {
	    		choices.push(format!("{};{}", pair.split(";").collect::<Vec<&str>>()[0], pair.split(";").collect::<Vec<&str>>()[1]));
				weights.push(pair.split(";").collect::<Vec<&str>>()[2].parse::<i32>().ok().unwrap());
	    	}
	    }
	    let mut rng = thread_rng();
	    if weights.len() > 0 {
	    	let dist = WeightedIndex::new(&weights).ok().unwrap();
	    	prev_char = choices[dist.sample(&mut rng)].split(";").collect::<Vec<&str>>()[1].to_string();
	    }
		else {
			prev_char = values.lines().collect::<Vec<&str>>()[rng.sample(Uniform::new(0, values.lines().collect::<Vec<&str>>().len()))].split(";").collect::<Vec<&str>>()[1].to_string();
		}
	    println!("{}", prev_char);
		prev_char = prev_char.replace("\n", "");
	    index+=1;
	}
}
