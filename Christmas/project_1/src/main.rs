use std::io;
use std::fs::File;
use std::io::Write;

fn main() {
    struct Company {
        username: String,
        password: String,
        shares: i32,
        liabilities: i32,
        founding_date: i32,
        percentage_leverage: f64,
    }

    let cadbury = Company {
        username: String::from("cadbury"),
        password: String::from("password"),
        shares: 15_000_000,
        liabilities: 5_500_000,
        founding_date: 1965,
        percentage_leverage: 5_500_000 as f64 / 15_000_000 as f64,
    };

    let champion = Company {
        username: String::from("champion"),
        password: String::from("password"),
        shares: 25_000_000,
        liabilities: 8_000_000,
        founding_date: 1974,
        percentage_leverage: 8_000_000 as f64 / 25_000_000 as f64,
    };

    let dangote = Company {
        username:String::from("dangote"),
        password:String::from("password"),
        shares: 18_000_000,
        liabilities: 10_000_000,
        founding_date: 1970,
        percentage_leverage: 10_000_000 as f64 / 18_000_000 as f64,
    };
    let flourMills = Company {
        username:String::from("flourMills"),
        password:String::from("password"),
        shares: 32_000_000,
        liabilities: 4_000_000,
        founding_date: 1960,
        percentage_leverage: 4_000_000 as f64 / 32_000_000 as f64,
    };
     let nestle = Company {
        username:String::from("nestle"),
        password:String::from("password"),
        shares: 8_000_000,
        liabilities: 1_500_000,
        founding_date: 1961,
        percentage_leverage: 1_500_000 as f64 / 8_000_000 as f64,
    };

    let honeywell = Company {
        username:String::from("honeywell"),
        password:String::from("password"),
        shares: 34_000_000,
        liabilities: 9_000_000,
        founding_date: 1906,
        percentage_leverage: 9_000_000 as f64 / 34_000_000 as f64,
    };

     let unilever = Company {
        username:String::from("unilever"),
        password:String::from("password"),
        shares: 37_000_000,
        liabilities: 11_000_000,
        founding_date: 1923,
        percentage_leverage: 11_000_000 as f64 / 37_000_000 as f64,
    };
         
     let nigerianBreweries = Company {
        username:String::from("breweries"),
        password:String::from("password"),
        shares: 30_000_000,
        liabilities: 12_000_000,
        founding_date: 1946,
        percentage_leverage: 12_000_000 as f64 / 30_000_000 as f64
    };

    let names = vec![
        "Cadbury Nigeria Plc",
        "Champion Breweries Plc",
        "Dangote Sugar Refinery Plc", 
        "Flour Mills Nigeria Plc", 
        "Nestle Nigeria Plc",
        "Unilever Nigeria Plc", 
        "Honeywell Nigeria Plc", 
        "Nigerian Breweries Plc"
    ];
    let companies: Vec<Company> = vec![cadbury, champion, dangote, flourMills, nestle, unilever, honeywell, nigerianBreweries];

    let file_name = "Ryan's companies.txt";
    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(b"           Ryan's companies.txt\n").expect("Error writing to file");
    file.write_all(
        format!(
            "{:<20} {:<20} {:<30} {:<30} {:<30}\n",
            "Company",
            "Founding Date",
            "Company Shares",
            "Company Liabilities",
            "Percentage Leverages"
        )
        .as_bytes(),
    )
    .expect("Error writing to file");

    for n in 0..names.len() {
        file.write_all(
            format!(
                "{:<20} {:<20} {:<30} {:<30} {:.2}\n",
                names[n],
                companies[n].founding_date,
                companies[n].shares,
                companies[n].liabilities,
                companies[n].percentage_leverage
            )
            .as_bytes(),
        )
        .expect("Error writing to file");
    }

    println!("Company details written to {}", file_name);

    loop {
        let mut username = String::new();
        println!("Please input your username:
            'cadbury' to access data for Cadbury
            'champion' to access data for Champion
            'dangote' to access data for Dangote
            'flourMills' to access data for Flour Mills
            'nestle' to access data for Nestle
            'unilever' to access data for Unilever
            'honeywell' to access data for Honeywell
            'breweries' to access data for Nigerian Breweries");
        io::stdin().read_line(&mut username).expect("Could not read input");
        let username1: &str = username.trim();

        if username1.chars().count() < 3 || username1.chars().count() > 8 {
            println!("Invalid username");
        } else {
            let mut password = String::new();
            println!("Please input a password:
            'cadbury' to access data for Cadbury
            'champion' to access data for Champion
            'dangote' to access data for Dangote
            'flourMills' to access data for Flour Mills
            'nestle' to access data for Nestle
            'unilevr' to access data for Unilever
            'honeywell' to access data for Honeywell
            'breweries' to access data for Nigerian Breweries");
            io::stdin().read_line(&mut password).expect("Could not read input");
            let passwordX: &str = password.trim();

            if passwordX.chars().all(|a: char| a.is_ascii_lowercase() || a.is_digit(10)) {
                println!("Incorrect password");
            } else {
                password = passwordX.to_string();
            }

            let mut check = false;

            for company in companies.iter() {
                if company.username == username1 && company.password == password {
                    check = true;
                    break;
                }
            }

            if check {
                break;
            } else {
                println!("Wrong password or username");
                return;
            }
        }
    }
}