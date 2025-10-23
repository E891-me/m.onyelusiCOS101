//Rust program to find the roots of
//a quadratic equation

use std::io;
  fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not avalid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let discriminant:f64 = b.powf(2.00) - (4.00 * a * c);
        if discriminant > 0.00
    {
        println!("two distinct roots");
        let root_1:f64 = -b + (discriminant.sqrt()) / (2.00 * a);
        let root_2:f64 = -b - (discriminant.sqrt()) / (2.00 * a);

        println!("The two roots are {}, and {}", root_1,root_2);
    }
        if discriminant == 0.00
    {
        println!("one real roots");
        let root_1:f64 = -b + (discriminant.sqrt()) / (2.00 * a);

        println!("The root is {}", root_1);
    }
        if discriminant < 0.00
    {
        println!("no real roots");
    }
    
    println!("Discriminant: {}", discriminant);
}