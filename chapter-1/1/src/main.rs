mod chap;
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
    let mut sum = 0;
    let size = array.len();
    for n in &array{
        sum = sum + n;
    }
    match size{
        0 => println!("divide error"),
        _ => {
            let avg = (sum as f64)/(size as f64);
            println!("avg: {:?}", avg);
        }
    };

    //copy to another arra
    //let array2 = array.clone();
    let mut array2: Vec<u32> =  Vec::with_capacity(array.len());
    for n in &array{
        array2.push(n.clone())
    }

    //revrse theelements within the array
    let len = array2.len();
    for n in (0..len/2){
        array2.swap(n, len-n-1);
    }
    // matrix matrix multiplication square matrices

    let a = vec![vec![1,1], vec![2,2]];
    let b = vec![vec![3,3], vec![4,4]];
    let mut c = vec![[0,0], [0,0]];
    for j in 0..2{
        for l in 0..2{
            for k in 0..2{
                let new_value;
                let value = a.get(j).unwrap().get(k).unwrap() *
                    b.get(k).unwrap().get(l).unwrap();
                { // its own scope for reading
                    let current_value = c.get(j).unwrap().get(l);
                    new_value = match current_value{
                        Some(current) => {current+value}
                        None => value
                    };
                }
                c.get_mut(j).unwrap()[l]= new_value;
            }
        }
    }
    println!("matrix {:?}", c);
    let y=chap::abs(0-1);
    println!("abs {:?}", y);

    let y=chap::abs_float(0.0-1.1);
    println!("abs {:?}", y);
    for n in 2..24{
        println!("{}: {:?}",n, chap::is_prime(n))
    }
    println!("sqrt {:?}", chap::sqrt(33434.4));
    println!("hypo {:?}", chap::hypo(12.0,5.0));
    println!("h{:?}", chap::h(3));
    let find_me = vec![1,2,3,4,5,6,7,8,9,10];
    println!("rank{:?}", chap::rank(&find_me, 8));
    println!("uni1 {:?}", chap::uni1(3,16));

}
