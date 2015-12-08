fn main() {
    println!("Hello, world!");
    // code fragments page #21

    //find the max of the array values
    let array: Vec<u32> = vec![3,6,5];
    let mut max =  array.get(0).unwrap();
    for n in &array{
        if n> max{
            max = n;
        }
    }
    println!("max: {:?}", max);

    //Compute the average
    let mut sum = 0 as u32;
    let size = array.len() as u32;
    for n in &array{
        sum = sum + n;
    }
    match size{
        0 => println!("divide error"),
        _ => {
            let avg:f64 = (sum as f64)/(size as f64);
            println!("avg: {:?}", avg);
        }
    };
}
