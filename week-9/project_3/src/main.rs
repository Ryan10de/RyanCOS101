use std::io::Write;
use std::io::Read;

fn main() {
    let mut input1 = String::new();
    println!("Enter in the number of people:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let n_o_p = input1.trim().parse().expect("Invalid input");

    for index in 0..n_o_p{
    let mut comm : Vec<String> = Vec::new();
    let mut nig = String::new();
    println!("What is the name of the comissioner?");
    std::io::stdin().read_line(&mut nig).expect("Failed to read input");
    let n_o_c:String = nig; 
    comm.push(n_o_c.clone());

    let mut mn : Vec<String> = Vec::new();
    let mut m_n = String::new();
    println!("What is your ministry?");
    std::io::stdin().read_line(&mut m_n).expect("Failed to read input");
    let min:String = m_n;
    mn.push(min.clone());

    let mut geo : Vec<String> = Vec::new();
    let mut g_z = String::new();
    println!("What is your geopolitical zone?");
    std::io::stdin().read_line(&mut g_z).expect("Failed to read input");
    let g:String = g_z;
    geo.push(g.clone());

    let mut file = std::fs::File::create("EFCC.txt").expect("create failed");

    file.write_all("\tEconomic and Financial Crimes Commission\t".as_bytes()).expect("write failed");
    file.write_all("\nNAME OF COMMISIONER\t\t\t\tMINISTRY\t\t\tGEOPOLITICAL ZONES\t".as_bytes()).expect("write failed");
    file.write_all(n_o_c.as_bytes()).expect("write failed");
    file.write_all(min.as_bytes()).expect("write failed");
    file.write_all(g.as_bytes()).expect("write failed");

    println!("\nData written into file.");
}
}