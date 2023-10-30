

mod calc;
mod input;

fn main() {
    let mut x = 0.0;
    let mut cur_res: f64 = 0.0;
    println!("{}", x);
    let user_input = input::num_in();
    x = user_input.operand;
    if user_input.operator == '+'{
        calc::add_mut_f64(&mut cur_res, &x)
    }
    println!("{}", cur_res);

}
