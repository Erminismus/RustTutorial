pub fn strg(){
	let mut st1: String = String::new(); //this string needs to be mutable 
	st1.push('A'); //single quotes are 
	st1.push_str("word");

	for word in st1.split_whitespace(){
		println!("{}", word);
	}

	let st2:String = st1.replace("A","Another");
	println!("{}",st2);

}