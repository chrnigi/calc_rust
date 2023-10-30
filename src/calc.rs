pub fn add_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 + *num_2;
}

pub fn subtract_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 - *num_2;
}

pub fn mult_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 * *num_2;
}

pub fn divi_mut_f64(num_1: &mut f64, num_2: &f64) {
    *num_1 = *num_1 / *num_2;
}