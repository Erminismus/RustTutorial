use rand::Rng;

pub fn rnd() -> f32{
	let x: f32 = rand::thread_rng().gen();
	let y: f32 = rand::thread_rng().gen();

	return x + y; 
}