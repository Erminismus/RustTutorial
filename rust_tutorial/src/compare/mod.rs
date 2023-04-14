use std::cmp::Ordering;


pub fn cmpre(){
	let my_age: i8 = 18;
	let voting_age: i8 = 18;
	match my_age.cmp(&voting_age){
		Ordering::Less => println!("Cannot vote"),
		Ordering::Greater => println!("Can vote"),
		Ordering::Equal => println!("You gained voting rights!")
	}
}


