use std::io::Read;
use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("Nigeria_Brewery_Limited.txt").expect("create failed");
    file.write_all("  Lager               Stout            Non-Alcoholic
    33 Export          Legend             Maltina
    Desperados         Turbo King         Amstel Malta
    Goldberg           Williams           Malta Gold
    Gulder                                Fayrouz
    Heineken                                           
    Star                                            \n"
                    .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    println!("\nData written to file. ");
  
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
