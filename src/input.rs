use std::io;

pub fn num_in() -> f64 {
    let mut input: String = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid number!");
    let input = input.trim().parse::<f64>().expect("Failed parse!");
    return input;
}