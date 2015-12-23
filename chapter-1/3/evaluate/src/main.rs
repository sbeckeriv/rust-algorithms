use std::io;
// page 129
fn main() {
    let mut string = String::new();
    let mut commands = Vec::new();
    let mut values = Vec::new();
    io::stdin().read_line(&mut string).unwrap();
    for token in string.split_whitespace() {
        match token {
            "(" | "+" | "-" => commands.push(token),
            ")" => {
                let op = commands.pop().unwrap();
                let mut value = values.pop().unwrap();
                match op {
                    "+" => value = value + values.pop().unwrap(),
                    "-" => value = value - values.pop().unwrap(),
                    _ => println!("Unknown op {}", op),
                }
                values.push(value);
            }
            _ => {
                match token.parse::<f64>() {
                    Ok(num) => values.push(num),
                    Err(_) => println!("Could not process {}", token),
                }
            }
        }
    }

    println!("{:?}", values.pop().unwrap())
}
