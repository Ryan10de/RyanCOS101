use std::io;

fn get_name_and_age_of_siblings(sibling_amount: i32)
{
    for _siblings in 0..sibling_amount
    {
        println!("Its first name?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _sibling_name = input.trim();

        println!("How old is It?");
        let mut input_1 = String::new();
        io::stdin().read_line(&mut input_1).expect("Failed to read input");
        let _sibling_age = input_1.trim();

        if _sibling_age > "18"
        {
            get_sibling_info()
        }
        else if _sibling_age < "18" 
        {
            sibling_info_2()
        }

        let sibling_arr = [_sibling_name, _sibling_age];

        for s in 0..sibling_amount
        {
            println!("{} Name:, Age: {}", s, sibling_arr[s]);
        }
    }
}

fn get_sibling_info()
{
    println!("Is he/she single or married? (S/M)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _choice_martial_status = input.trim();

    if _choice_martial_status == "S"
    {
        println!("Is he/she a student or a worker? (S/W)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _choice_occupation = input.trim();

        if _choice_occupation == "S"
        {
            println!("What is he/her University?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let _sibling_college = input.trim();

            println!("What is he/her Course of study?");
            let mut input_1 = String::new();
            io::stdin().read_line(&mut input_1).expect("Failed to read input");
            let _sibling_course = input_1.trim();

        }
    }
    else if _choice_martial_status == "M"
    {
        println!("Does he/her have any offspring? (Y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _choice = input.trim();

        println!("Which city does he/her family live?");
        let mut input_1 = String::new();
        io::stdin().read_line(&mut input_1).expect("Failed to read input");
        let _sibling_city = input_1.trim();
    }
}

fn sibling_info_2()
{
    println!("Have he/she written WAEC? (Y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _choice = input.trim();

    if _choice == "Y"
    {
        println!("Which Secondary School did he/she attend?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _sibling_school = input.trim();
    }
    else
    {
        println!("What is his/her current class level?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _sibling_class = input.trim();
    }
}

fn main() 
{
    println!("Hey, What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let _name = name.trim();

    println!("Do you have any siblings {}? (Y/N)", name);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _sibling = input.trim();

    if _sibling == "Y"
    {
        println!("How many siblings do you have?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _amount_sibling:i32 = input.trim().parse().expect("Invalid input");

        get_name_and_age_of_siblings(_amount_sibling)
    }
}