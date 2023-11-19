fn main(){
    println!("are these stings equal");

let apple: String = String::from("apple");
let eve: String = String::from("apple");
println!("{}", apple.eq(&eve));
let num:i32 = 32;
let num2:i32 = 64;

    if apple == "apple" && num == 32 && num2 == 64 {
        println!("yh");
    }
    else {
        println!("nah");
    }
}