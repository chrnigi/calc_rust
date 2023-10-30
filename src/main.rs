

mod calc;
mod input;

fn main() {
    let mut x: f64;
    let mut cur_res: f64 = 0.0;
    //println!("{}", x);
    let mut user_input: input::UserIn;
    
    println!("Current result: {:.3}", cur_res);

    loop {
        user_input = input::num_in();
        x = user_input.operand;
        match user_input.operator {
            '+' => calc::add_mut_f64(&mut cur_res, &x),
            '-' => calc::subtract_mut_f64(&mut cur_res, &x),
            '*' => calc::mult_mut_f64(&mut cur_res, &x),
            '/' => calc::divi_mut_f64(&mut cur_res, &x),
            'q' => break,
            _ => println!("{} is not a valid operator!", user_input.operator),
        }
        println!("Current result: {:.3}", cur_res);

    }
    println!("Final result: {:.3}", cur_res);

}
