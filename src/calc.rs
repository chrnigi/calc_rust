
use crate::input::UserIn;

fn add_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 + *num_2;
}

fn subtract_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 - *num_2;
}

fn mult_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 * *num_2;
}

fn divi_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 / *num_2;
}

fn square_root_f64(num: &mut f64) {
    *num = f64::sqrt(*num);
}

pub fn do_next_op(mut res: &mut f64, user_input: &UserIn) {
    match user_input.operator {
        '+' => add_mut_f64(&mut res, &user_input.operand),
        '-' => subtract_mut_f64(&mut res, &user_input.operand),
        '*' => mult_mut_f64(&mut res, &user_input.operand),
        '/' => divi_mut_f64(&mut res, &user_input.operand),
        '#' => square_root_f64(&mut res),
        _ => println!("{} is not a valid operator!", user_input.operator),
    }
}