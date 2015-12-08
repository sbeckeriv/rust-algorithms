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

    //copy to another arra
    //let array2 = array.clone();
    let mut array2: Vec<u32> =  Vec::with_capacity(array.len());
    for n in (0..array.len()){
        array2.insert(n,array.get(n).unwrap().clone())
    }

    //revrse theelements within the array
    let len = array2.len();
    for n in (0..len/2){
        array2.swap(n, len-n-1);
    }
    // matrix matrix multiplication square matrices

    let a = vec![vec![1,1], vec![2,2]];
    let b = vec![vec![3,3], vec![4,4]];
    let mut c = vec![Vec::<i32>::with_capacity(2), Vec::<i32>::with_capacity(2)];
    let c_len = c.len();
    for j in 0..a.len(){
        for l in 0..b.len(){
            for k in 0..c_len{
                let new_value;
                let value = a.get(j).unwrap().get(k).unwrap() *
                    b.get(k).unwrap().get(l).unwrap();
                {
                    let current_value = c.get(j).unwrap().get(l);
                    new_value = match current_value{
                        Some(current) => {current+value as i32}
                        None => value
                    };
                }
                c.get_mut(j).unwrap().insert(l, new_value)
            }
        }
    }
    println!("matrix {:?}", c)

}
