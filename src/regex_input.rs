use std::io;

pub struct UserIn {
    pub operator: char,
    pub operand: f64,
}

pub fn scan_data() -> UserIn {

    let mut input = String::new();
    let num: f64;
    let op: char;

    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input");

    let re = regex::Regex::new(r"(?<operator>.{1})\s*(?<operand>[0-9]+\.?[0-9]*)?\s*(?<check>.*)").unwrap();
    
    let caps = re.captures(input.as_str()).unwrap();
    
    op = caps.name("operator").unwrap().as_str().parse::<char>().unwrap_or('+');

    let check_input = UserIn {
        operator: '+',
        operand: 0.0,
    };
    
    let check = caps.name("check").map_or("", |m| m.as_str());
    match check {
        "" => (),
        _ => return check_input,
    }

    match op {
        'q' => num = 0.0,
        '#' => num = 0.0,
        '!' => num = 0.0,
        '%' => num = 0.0,
        _ => num = caps.name("operand").map_or("", |m| m.as_str()).parse::<f64>().unwrap_or(0.0),
    }

    let user_input = UserIn {
        operator: op,
        operand: num,
    };
    
    return user_input;
}