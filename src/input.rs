use std::io;
//use std::io::prelude::*;
//use regex::Regex;

pub struct UserIn {
    pub operator: char,
    pub operand: f64,
}

pub fn num_in() -> UserIn {
    let mut input_op: String = String::new();

    let temp_op: char;
    let temp_num: f64;
    io::stdin()
        .read_line(&mut input_op)
        .expect("Invalid input!");
    
    temp_op = input_op[0..1].trim().parse::<char>().expect("Failed parse!");
    
    match temp_op {
        
        'q' => temp_num = 0.0,
        '#' => temp_num = 0.0,
        '%' => temp_num = 0.0,
        '!' => temp_num = 0.0,
        _ => temp_num = input_op[1..input_op.len()].trim().parse::<f64>().expect("Failed parse!"),

    }
        
    let user_input = UserIn { operator: temp_op, operand: temp_num};
    
    return user_input;
}