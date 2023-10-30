

mod calc;
mod input;

fn main() {
    let mut x = 0.0;
    let mut cur_res: f64 = 0.0;
    println!("{}", x);
    let user_input = input::num_in();
    x = user_input.operand;

    match user_input.operator {
        '+' => calc::add_mut_f64(&mut cur_res, &x),
        '-' => calc::subtract_mut_f64(&mut cur_res, &x),
        '*' => calc::mult_mut_f64(&mut cur_res, &x),
        '/' => calc::divi_mut_f64(&mut cur_res, &x),
        _ => println!("{} is not a valid operator!", user_input.operator),
    }
    println!("{}", cur_res);

}
