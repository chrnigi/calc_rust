mod calc;
fn main() {
    let x = 3.2;
    let y = 3.4;
    let z = calc::add_f64(&x, &y);
    println!("{} + {} = {}", x, y, z);
}
