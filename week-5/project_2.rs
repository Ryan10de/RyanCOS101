use std::io;

fn main()
{
let mut input1 = String::new();

println!("\nEnter Your Age");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let age:f32 =input1.trim().parse().expect("Not a valid string");

if age >= 40.00
{
println!("You are experienced and your incentive is 1,560,000.00");
}
else if age > 30.00 && age <= 40.00
{
println!("You are experienced and your incentive is 1,480,000.00");
}
else if age > 10.00 && age <= 28.00
{
println!("You are experienced and your incentive is 1,300,000.00");
}
else
{
println!("You are inexperienced and your incentive is 100,000.00");
}
}