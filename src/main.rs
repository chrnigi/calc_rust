mod calc;
fn main() {
    let mut x = 3.2;
    println!("{}", x);
    let y = 3.4;
    calc::add_mut_f64(&mut x, &y);
    println!("{}", x);

}
