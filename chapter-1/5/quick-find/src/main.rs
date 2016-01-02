mod union_find;
mod stdin_lines;
fn main() {
    let reader = stdin_lines::StdinLines::new();
    let mut count: Option<usize> = None;
    let mut union;
    for line in reader {
        if line != "\n" {
            if count.is_some() {
                let nums: Vec<&str> = line.split_whitespace().collect();
                let left_num: usize = nums[0].parse::<usize>().unwrap();
                let right_num: usize = nums[1].parse::<usize>().unwrap();
            }else{
                let trimmed = line.trim();
                count = Some(trimmed.parse::<usize>().unwrap());
                union = union_find::UF::new(&count.unwrap());
            }
        } else {
            break;
        }
    }
    println!("Hello, world!");
}
