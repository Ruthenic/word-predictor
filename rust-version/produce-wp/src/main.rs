extern crate produce_wp;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main(){
	println!("How many words would you like to make? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok().unwrap();
    line.pop();
    //println!("{}", line);
    let mut num_of_chars = line.parse::<i32>().ok().unwrap();
    num_of_chars = num_of_chars - 1;
    println!("Which word would you like to start with? ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line.pop();
    let mut f = File::open("counts.txt").ok().unwrap();
    //let mut f = File::open("counts.txt").expect("unable to find counts.txt");
    let mut file_string = String::new();
    f.read_to_string(&mut file_string);
    let values = file_string;
    produce_wp::gen(line, num_of_chars, values)
}
