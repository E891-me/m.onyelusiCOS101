fn main() {
    
    //Create two vector
    let v = vec![1,2,3,4,5,6,7,8];
    let d = vec![7,8,9,10,11,12,13,14];

    //Use a for loop to add elements of the vector
    for index in 0..8 {
        let sum = v[index] + d[index];
        println!("{:?}", sum);
    }
}
