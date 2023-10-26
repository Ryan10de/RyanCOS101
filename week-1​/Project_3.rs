fn main() {
let p:f64  =  210_000.0;
let r:f64  = 5.0;
let n:f64 = 3.0;

// depreciation
let a = p*(1.0 - r  / 100.00).powf(n);
println!("Amount is {}", a);
let de = p-a;
println!("Depreciation is {}", de);
    
}