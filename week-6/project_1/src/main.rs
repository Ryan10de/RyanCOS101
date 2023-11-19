use std::io;

fn main() {
    let mut count = 0;

    loop {
        if count >= 500 {
            println!("Voting has ended.");
            break;
        }

        let mut class = String::new();
        let mut cgpa = String::new();
        let mut is_class_rep = String::new();
        let mut name = String::new();
        let mut email = String::new();
        let mut department = String::new();
        let mut sOO = String::new();

        println!("Please enter your class (100, 200, 300, 400):");
        io::stdin().read_line(&mut class).expect("Failed to read line");

        println!("Please enter your CGPA (as a float):");
        io::stdin().read_line(&mut cgpa).expect("Failed to read line");

        println!("Are you a class rep? (true/false):");
        io::stdin().read_line(&mut is_class_rep).expect("Failed to read line");

        println!("What is your name?");
        io::stdin().read_line(&mut name).expect("Failed to read input");

        println!("What is your school e-mail?");
        io::stdin().read_line(&mut email).expect("Failed to read input");

         println!("What is your department?");
        io::stdin().read_line(&mut department).expect("Failed to read input");
        
        println!("What is your State of origin?");
        io::stdin().read_line(&mut sOO).expect("Failed to read input");

        

        let class: i32 = class.trim().parse().expect("Please enter a valid integer");
        let cgpa: f32 = cgpa.trim().parse().expect("Please enter a valid float");
        let is_class_rep: bool = is_class_rep.trim().parse().expect("Please enter true or false");
        let name = name.trim();
        let email = email.trim();
        let department = department.trim();
        let sOO = sOO.trim();

        if is_class_rep && class != 100 && cgpa >= 4.0 && class >= 100 {
            println!("You can vote");
            println!("{}", name);
            println!("{}", email);
            println!("{}", department);
            println!("{}", sOO);


        } else {
            println!("Sorry, you can't vote");
        }

        count += 1;
    }
}
