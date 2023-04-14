
pub fn isd(){
	let x:i32 = 24;

	match x{
		1..3 => println!("{}", false),
		_ => println!("{}", true)
	};
}