mod stdin_lines;
fn main() {
    let reader = stdin_lines::StdinLines::new();
    let mut queue = Vec::new();
    println!("Please enter numbers (blank line to stop):");
    for line in reader{
        if line != "\n" {
            let trimmed = line.trim();
            match trimmed.parse::<u64>(){
                Ok(num) => {
                    queue.push(num);
                }
                Err(_) => {
                    println!("Could not parse {}", line);
                }
            }
        } else{
            break;
        }
    }
    while let Some(num) = queue.pop(){
        println!("Dequeue {:?}", num);
    }
}
