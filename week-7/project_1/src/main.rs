use std::io;

   fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value: f64 = input.trim().parse().expect("Invalid input");
    return value;
   }

   // area of trapezium
   fn trapezium() {
    println!("\n Area of Trapezium");
    let h = get_input("Enter the height: ");
    let b1 = get_input("Enter the first base: ");
    let b2 = get_input("Enter the second base: ");

    let area = (h / 2.0) * (b1 + b2);
    println!("The Area of the Trapezium is {}", area);
   }

   // area of a rhombus
   fn rhombus() {
    println!("\n Area of Rhombus");
    let d1 = get_input("Enter the first diagonal: ");
    let d2 = get_input("Enter the second diagonal: ");

    let area = 0.5 * d1 * d2;
    println!("The Area of the Rhombus is {}", area);
   }

   // area of parallelogram
   fn parallelogram() {
    println!("\n Area of Parallelogram");
    let b = get_input("Enter value for the base: ");
    let al = get_input("Enter value for the altitude: ");

    let area = b * al;
    println!("The Area of the Parallelogram is {}", area);
   }

   // area of cube
   fn cube() {
    println!("\n Area of Cube");
    let l = get_input("Enter the length of one side: ");

    let area = 6.0 * (l).powf(2.0);
    println!("The Area of the cube is {}", area);
   }

   // volume of cylinder
   fn cylinder() {
    println!("\n Volume of Cylinder");
    let r = get_input("Enter the value of the radius: ");
    let h = get_input("Enter the value of the height: ");

    let volume = 3.142 * (r).powf(2.0) * h;
    println!("The Volume of Cylinder is {}", volume);
   }

   //invoking the functions
   fn main() {
       loop{
        println!("\n          MTH 101 Calculator          ");
        println!("            Select a calculation:       ");
        println!("           1. Area of Trapezium         ");
        println!("           2. Area of Rhombus           ");
        println!("           3. Area of Parallelogram     ");
        println!("           4. Area of Cube              ");
        println!("           5. Volume of Cylinder        ");
        println!("           0. Exit                      ");
        println!("<======================================>");
        println!("           Enter your choice (0-5):     ");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");

        let option:u32 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!!");
                continue;
            }
        };

        if option == 1 {
            trapezium();
        }else if option == 2 {
            rhombus();
        }else if option == 3 {
            parallelogram();
        }else if option == 4 {
            cube();
        }else if option == 5 {
            cylinder();
        }else if option == 0 {
            println!("Thank you for using the Calculator. Peace!!");
            break;
        }else {
            println!("Invalid Entry!! Select fromoptions 0-5. ");
        }
      }
   }