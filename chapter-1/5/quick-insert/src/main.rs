extern crate time;
mod union_find;
mod stdin_lines;
fn main() {
    let reader = stdin_lines::StdinLines::new();
    let mut count: Option<usize> = None;
    let mut union= union_find::UF::new(&0);
    let t = time::now();
    for line in reader {
        let trimmed = line.trim();
        if trimmed != "" {
            if count.is_some() {
                let nums: Vec<&str> = trimmed.split_whitespace().collect();
                let left_num: usize = nums[0].parse::<usize>().unwrap();
                let right_num: usize = nums[1].parse::<usize>().unwrap();
                if !union.connected(left_num, right_num){
                    union.union(left_num, right_num);
                }
            }else{
                count = Some(trimmed.parse::<usize>().unwrap());
                union = union_find::UF::new(&count.unwrap());
            }
        } else {
            break;
        }
    }
    println!("{} node/s in {:?}", union.count, time::now()-t);
}
