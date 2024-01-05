use std::io::Write;

fn main() {
    let lager = vec!["\n33 Export".to_string(),"\nDesperados".to_string(),"\nGoldberg".to_string(),"\nGulder".to_string(),"\nHeineken".to_string(),"\nStar\n".to_string()];
    let stout = vec!["\nLegend".to_string(),"\nTurbo King".to_string(),"\nWilliams\n".to_string()];
    let non_alcoholic = vec!["\nMaltina".to_string(),"\nAmstel Malta".to_string(),"\nMalta Gold".to_string(),"\nFayrouz\n".to_string()];

    let mut file = std::fs::File::create("Nigerian Breweries Plc.txt").expect("create failed");
    file.write_all("Welcome to the Nigeria Breweries Plc.\nWe have some amazing drinks for you to try out.\nHere they are under their various categories:\n".as_bytes()).expect("write failed");

    file.write_all("\nLager:".as_bytes()).expect("write failed");
    for i in lager {
         file.write_all(i.as_bytes()).expect("write failed");
    }

    file.write_all("\nStout:".as_bytes()).expect("write failed");
    for j in stout{
        file.write_all(j.as_bytes()).expect("write failed");
    }

    file.write_all("\nNon-Alcoholic:".as_bytes()).expect("write failed");
    for k in non_alcoholic{
        file.write_all(k.as_bytes()).expect("write failed");
    }
    println!("\nData written into file.");

}