mod bags;
mod stdin_lines;
fn main() {
    let mut numbers = bags::Ziploc::<f64>::new();
    let stdinlines = stdin_lines::StdinLines::new();
    for line in stdinlines{
        if line != "\n" {
            let trimmed = line.trim();
            let number = trimmed.parse::<f64>().unwrap();
            numbers.add(&number);
        } else{
            break
        }
    }

    //sum moves numbers this line first
    let n = numbers.size() as f64;
    {
        let mut sum = 0 as f64;
        for item in numbers{
            sum = sum + item;
        }

        println!("Sum:  {:?}", sum);
        let mean = sum as f64/n;
        println!("Mean: {:?}", mean);
    }
    let mut std_sum = 0 as f64;
    for item in numbers{
        std_sum = std_sum + item;
    }
    /*
       let std_sum = numbers.into_iter().fold(0 as f64, |acc, item|
       acc + ((item as f64-mean) * (item as f64-mean))) as f64;
       let std = ((sum/(n-1 as f64)) as f64).sqrt();
       println!("Std : {:?}", std);
       */
}

