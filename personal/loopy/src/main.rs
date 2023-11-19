use std::io;
fn main() {

let mut input = String::new();

println!("\nEnter the max number range");
io::stdin().read_line(&mut input).expect("Not a valid string");
let n:i32 =input.trim().parse().expect("Not a valid string");    

    for i in 1..=n {
        for j in 1..=12 {
            if j == 1 {
                print!("{:>2} x{:>2} = {:>2}", i, j, i * j);
            } else {
                print!(" {:>2} x{:>2} = {:>2}", i, j, i * j);
            }
        }
        println!();
    }
}
