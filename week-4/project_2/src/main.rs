//Rust program to detremine annual
//incentive of employee

use std::io;
fn main() {
    let mut employee = String::new();
    let mut age = String::new();

    println!("Enter if employee is experienced or not");
    println!("Type either true or false");
    io::stdin().read_line(&mut employee).expect("Not a valid string");
    let experienced_status:String = employee.trim().parse().expect("Not a valid string"); 

    println!("Enter age");
    io::stdin().read_line(&mut age).expect("Not a valid age");
    let years_old:u32 = age.trim().parse().expect("Not avalid age");

    if experienced_status == "true" && years_old >= 40 {

        println!("The annual incentive of the employee is 1560000");
    }
    else if experienced_status == "true" && years_old >= 30 && years_old < 40 {

        println!("The annual incentive of the employee is 1480000");
    }
    else if experienced_status == "true" && years_old < 28 {

        println!("The annual incentive of the employee is 1300000");
    }
    else if experienced_status == "false" {

        println!("The annual incentive of the employee is 100000");
    }
    else {
    println!("Your values doesnt match any criteria");
}

}
