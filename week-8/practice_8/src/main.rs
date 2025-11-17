fn main() {
    //initialize a mutable tuple
    let mut mountain_heights = ("Everest",8848, "Fish tail", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    // change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Esther";
    mountain_heights.3 = 2009;

    println!("Changed tuple = {:?}", mountain_heights);
}
