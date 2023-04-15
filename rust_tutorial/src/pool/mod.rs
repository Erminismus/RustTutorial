pub fn pool(){
	let arr_1:[i32;3] = [1,2,3];

	let mut loop_idx:usize = 0;

	loop{
		
		if arr_1[loop_idx] == 3{
			println!("Val: {}",arr_1[loop_idx]);
			break;
		

		}
		loop_idx += 1; 
	}
}