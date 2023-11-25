use std::io;

fn get_area_of_trap(height:f64, base_1:f64, base_2:f64)
{
    let area = height / 2.0 * (base_1 + base_2);
    println!("The area for Trapezium = {}", area);
}

fn get_area_of_rhomb(diagonal_1:f64, diagonal_2:f64)
{
    let area = diagonal_1 * diagonal_2 / 2.0;
    println!("The area for Rhombus = {}", area);
}

fn get_area_of_para(base:f64, altitude:f64)
{
    let area = base * altitude;
    println!("The area for Parallelogram = {}", area);
}

fn get_area_of_cube(length: i32)
{
    let area = 6 * length ^ 2;
    println!("The area for Cube = {}", area);
}

fn get_volume_of_cyli(height:f64, radius:f64)
{
    let pi:f64 = 22.0 / 7.0;
    let area = pi * height * radius * radius;
    println!("The volume for Cylinder = {}", area);
}


fn main() 
{
    println!("Welcome to Calculations:");
    println!("Enter 1 if to get the area of Trapezium");
    println!("Enter 2 if to get the area of Rhombus");
    println!("Enter 3 if to get the area of Parallelogram");
    println!("Enter 4 if to get the area of Cube");
    println!("Enter 5 if to get the volume of Cylinder");
    let mut choice = String::new();
    println!("Enter your choice from 1 ~ 5");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let _choice:i32 = choice.trim().parse().expect("Failed to input");

    if _choice == 1
    {
        println!("Enter height");
        let mut height = String::new();
        io::stdin().read_line(&mut height).expect("Failed to read input");
        let _height:f64 = choice.trim().parse().expect("Failed to input");

        println!("Enter base 1");
        let mut base_1 = String::new();
        io::stdin().read_line(&mut base_1).expect("Failed to read input");
        let _base_1:f64 = base_1.trim().parse().expect("Failed to input");

        println!("Enter base 2");
        let mut base_2 = String::new();
        io::stdin().read_line(&mut base_2).expect("Failed to read input");
        let _base_2:f64 = base_2.trim().parse().expect("Failed to input");

        get_area_of_trap(_height, _base_1, _base_2);
    }

    if _choice == 2
    {
        println!("Enter diagonal 1");
        let mut diagonal_1 = String::new();
        io::stdin().read_line(&mut diagonal_1).expect("Failed to read input");
        let _diagonal_1:f64 = diagonal_1.trim().parse().expect("Failed to input");

        println!("Enter diagonal 2");
        let mut diagonal_2 = String::new();
        io::stdin().read_line(&mut diagonal_2).expect("Failed to read input");
        let _diagonal_2:f64 = diagonal_2.trim().parse().expect("Failed to input");

        get_area_of_rhomb( _diagonal_1, _diagonal_2);
    }

    if _choice == 3
    {
        println!("Enter base ");
        let mut base = String::new();
        io::stdin().read_line(&mut base).expect("Failed to read input");
        let _base:f64 = base.trim().parse().expect("Failed to input");

        println!("Enter altitude");
        let mut altitude = String::new();
        io::stdin().read_line(&mut altitude).expect("Failed to read input");
        let _altitude:f64 = altitude.trim().parse().expect("Failed to input");

        get_area_of_para(_base, _altitude);
    }

    if _choice == 4
    {
        println!("Enter length of sides");
        let mut length = String::new();
        io::stdin().read_line(&mut length).expect("Failed to read input");
        let _length:i32 = length.trim().parse().expect("Failed to input");

        get_area_of_cube(_length);
    }

    if _choice == 5
    {
        println!("Enter height");
        let mut height = String::new();
        io::stdin().read_line(&mut height).expect("Failed to read input");
        let _height:f64 = choice.trim().parse().expect("Failed to input");

        println!("Enter radius");
        let mut radius = String::new();
        io::stdin().read_line(&mut radius).expect("Failed to read input");
        let _radius:f64 = radius.trim().parse().expect("Failed to input");

        get_volume_of_cyli(_height, _radius);
    }
}