fn main() {
    
    let b:(i32,&str,f64) = (30,"Esther",4.9);
    print(b);
}
 fn print(x:(i32,&str,f64)) {
    println!("Inside print methods");
    //assigns a tuple to distinct variables
    let (age,name,cgpa) = x;
    println!("Age is {} , Name is {}, cgpa is {}", age,name,cgpa);
 }