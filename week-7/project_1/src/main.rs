use std::io;
use std::f64::consts::PI;

fn trapezium_area() -> f64 {
    let mut input = String::new();

    println!("Enter base 1:");
    io::stdin().read_line(&mut input).unwrap();
    let base1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter base 2:");
    io::stdin().read_line(&mut input).unwrap();
    let base2: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let mut input = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input).unwrap();
    let d1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input).unwrap();
    let d2: f64 = input.trim().parse().unwrap();

    0.5 * d1 * d2
}

fn parallelogram_area() -> f64 {
    let mut input = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input).unwrap();
    let base: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).unwrap();
    let altitude: f64 = input.trim().parse().unwrap();

    base * altitude
}

fn cube_surface_area() -> f64 {
    let mut input = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut input).unwrap();
    let side: f64 = input.trim().parse().unwrap();

    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let mut input = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    PI * radius * radius * height
}

fn main() {
    println!("===== Shape Calculator =====");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let result = match choice.trim() {
        "1" => trapezium_area(),
        "2" => rhombus_area(),
        "3" => parallelogram_area(),
        "4" => cube_surface_area(),
        "5" => cylinder_volume(),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    println!("Result = {:.2}", result);
}

