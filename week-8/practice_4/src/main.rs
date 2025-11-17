fn main() {
    

    // Name vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    //Age vector
    let age = vec![16,17,19,22,21,34,28,31];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        // iterating through 1 on the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
