fn main() {
    let name ="Onyelusi Esther";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni,addr);

    let department:&'static str ="Computer Science";
    let school:&'static str = "School of Science and Tehnology";
    println!("Department: {}, \nSchool: {}", department,school);
}
