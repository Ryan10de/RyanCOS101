use std::io;
fn main() {
	let mut time = String::new();
	let mut dist = String::new();

	println!("How long");
	io::stdin().read_line(&mut time).expect("Invalid");
	let a:f32 = time.trim().parse().expect("This not Valid");

	println!("How many miles");
	io::stdin().read_line(&mut dist).expect("Invalid");
	let b:f32 = dist.trim().parse().expect("This not Valid");
	let c:f32 = b*1.609;

let speed:f32 = c/a;

println!("Speed is: {} kmh", speed);
}
