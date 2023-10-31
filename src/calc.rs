//use crate::input::UserIn;
use crate::regex_input::UserIn;

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

fn square_root_mut_f64(num: &mut f64) {
    *num = f64::sqrt(*num);
}

fn reciprocal_mut_f64(num: &mut f64) {
    *num = 1.0 / *num;
}

fn negate_mut_f64(num: &mut f64) {
    *num = -*num;
}

fn power_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = (*num_1).powf(*num_2);
}

pub fn do_next_op(mut res: &mut f64, user_input: &UserIn) {
    
    if *res == 0.0 {
        match user_input.operator {
            '!' => return,
            _ => (),
        }
    } else if *res < 0.0 {
        match user_input.operator {
            '#' => return,
            _ => (),
        }
    }

    if user_input.operand == 0.0 {
        match user_input.operator {
            '/' => return,
            _ => (),
        }
    }

    match user_input.operator {
        '+' => add_mut_f64(&mut res, &user_input.operand),
        '-' => subtract_mut_f64(&mut res, &user_input.operand),
        '*' => mult_mut_f64(&mut res, &user_input.operand),
        '/' => divi_mut_f64(&mut res, &user_input.operand),
        '#' => square_root_mut_f64(&mut res),
        '!' => reciprocal_mut_f64(&mut res),
        '%' => negate_mut_f64(&mut res),
        '^' => power_mut_f64(&mut res, &user_input.operand),
        _ => println!("{} is not a valid operator!", user_input.operator),
    }
}