use std::io;


fn main(){
	const ONE_MIL:u32 = 1_000_000; //underscore to separate zeros
	const PI: f32 = 3.141592;
	let age: &str = "47";

	let mut age: u32 = age.trim().parse().expect(msg:"Age wasn't a number") //convert age to unsigned int
	age = age + 1;
	println!("I'm {} and I want ${}",age,ONE_MIL);

}
