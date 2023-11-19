use std::io;

fn main() {
    let mut count = 0;

    loop {
        if count >= 500 {
            println!("Voting has ended.");
            break;
        }

        println!("What is your name?");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let _name = name.trim();

        println!("How many papers have you published?");
        let mut published_papers = String::new();
        io::stdin().read_line(&mut published_papers).expect("Failed to read input");
        let _published_papers:i32 = published_papers.trim().parse().expect("Failed to input");

        if _published_papers > 3 && _published_papers <= 5
        {
            let incentive = 500_000;
            println!("{}, Your incentive is N{}",_name, incentive);
        }
        else if _published_papers > 5 && _published_papers < 10
        {
            let incentive = 800_000;
            println!("{}, Your incentive is N{}",_name, incentive);
        }
        else if _published_papers >= 10
        {
            let incentive = 1_000_000;
            println!("{}, Your incentive is N{}",_name, incentive);
        }
        else if _published_papers < 3 
        {
            let incentive = 100_000;
            println!("{}, Your incentive is N{}",_name, incentive);
        }
    }

        count += 1;
}
