#![allow(unused)]
//mod consts;
//mod max;
//mod rnd;
//mod ternary;
//use consts;
//mod match;
//mod compare;

mod pool;

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;



fn main() {
    println!("What is your name");
    let mut name: String = String::new(); //the :: access the methods of a type, 
                                             //without the need to have an instance of it
    let greeting: &str = "Nice to meet you!";

    io::stdin().read_line(&mut name).expect("Failed to read line"); // We will make name mutable
    
    println!("Hello {}!, {}",name.trim_end(),greeting); //trim end will remove the newline 

    //println!("{}",ternary::ternary_test(9));
    //match::isd();
    //compare::cmpre();
    //println!("{}",consts::add_nums(2.1,3.6));
    //max::show_max();
    pool::pool();


    //println!("{}",rnd::rnd())
}

