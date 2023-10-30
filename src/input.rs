use std::io;
use scanf::sscanf;

pub struct UserIn {
    pub operator: char,
    pub operand: f64,
}

pub fn num_in() -> UserIn {
    let mut input: String = String::new();
    

    let mut temp_op: char = '0';
    let mut temp_num: f64 = 0.0;
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid number!");
    sscanf!(&input, "{} {}", temp_op, temp_num);
    //let input = input.trim().parse::<f64>().expect("Failed parse!");
    let user_input = UserIn { operator: temp_op, operand: temp_num };
    
    return user_input;
}