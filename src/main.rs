use std::io;
mod calc;
mod input;

fn main() {
    let mut x = 0.0;
    println!("{}", x);
    let y = input::num_in();
    calc::add_mut_f64(&mut x, &y);
    println!("{}", x);

}
