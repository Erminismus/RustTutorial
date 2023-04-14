#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;

use std::cmp::Ordering;

fn main() {
    println!("What is your name");
    let mut name: String = String::new(); //the :: access the methods of a type, 
                                             //without the need to have an instance of it
    let greeting: &str = "Nice to meet you!";

    io::stdin().read_line(&mut name).expect("Failed to read line"); // We will make name mutable
    

    println!("Hello {}!, {}",name.trim_end(),greeting); //trim end will remove the newline 
}
