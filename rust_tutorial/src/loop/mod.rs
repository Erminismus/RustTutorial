pub fn pool(){
	let arr_1:[i32;3] = [1,2,3,4,5];

	let mut loop_idx:usize = 0;

	loop{
		if arr_1[loop_idx]%2 == 0{
			loop_idx += 1;
			continue;
		}	
		if arr_2[loop_idx] == 9{
			arr_2[loop_idx];
		}
		println!("Val: {}",arr_2[loop_idx] )
	}	loop_idx += 1;
}