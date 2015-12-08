fn main() {
    println!("Hello, world!");
    // code fragments page #21

    //find the max of the array values
    let array = vec![3,6,5];
    let mut max =  array.get(0).unwrap();
    for n in &array{
        if n> max{
            max = n;
        }
    }
    println!("{:?}", max);
}
