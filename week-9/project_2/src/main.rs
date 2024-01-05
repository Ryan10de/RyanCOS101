use std::io;
use std::io::Write;

fn main(){
    let mut input1 = String::new();
    println!("Enter in the number of students:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let n_o_s = input1.trim().parse().expect("Invalid input");

    let mut file = std::fs::File::create("Pausims.txt").expect("create failed");

    file.write_all("\tPAU SMIS\t".as_bytes()).expect("write failed");
    file.write_all("\nSTUDENT NAME\t\t\t\tMATRIC. NUMBER\t\t\tDEPARTMENT\t\t\tLEVEL\t".as_bytes()).expect("write failed");

    for index in 0..n_o_s{
    let mut sn : Vec<String> = Vec::new();
    let mut s_n = String::new();
    println!("What is your name, PAU student?");
    std::io::stdin().read_line(&mut s_n).expect("Failed to read input");
    let _student:String = s_n.clone(); 
    sn.push(_student);
    file.write_all("\n".as_bytes()).expect("write failed");
    file.write_all(s_n.as_bytes()).expect("write failed");

    let mut mn : Vec<String> = Vec::new();
    let mut m_n = String::new();
    println!("What is your matriculation number?");
    std::io::stdin().read_line(&mut m_n).expect("Failed to read input");
    let _mat:String = m_n.clone();
    mn.push(_mat);
    file.write_all("\t\t\t".as_bytes()).expect("write failed");
    file.write_all(m_n.as_bytes()).expect("write failed");

    let mut dp : Vec<String> = Vec::new();
    let mut d = String::new();
    println!("What is your department?");
    std::io::stdin().read_line(&mut d).expect("Failed to read input");
    let _dep:String = d.clone();
    dp.push(_dep);
    file.write_all(d.as_bytes()).expect("write failed");

    let mut level : Vec<String> = Vec::new();
    let mut ll = String::new();
    println!("What level are you in?");
    std::io::stdin().read_line(&mut ll).expect("Failed to read input");
    let _yr:i64 = ll.trim().parse().expect("Invalid input"); 
    level.push(_yr.to_string());
    file.write_all(ll.as_bytes()).expect("write failed");

    
    println!("\nData written into file.");

}
}