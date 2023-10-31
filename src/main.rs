mod calc;
mod regex_input;

fn main() {
    let mut cur_res: f64 = 0.0;
    
    let mut user_input: regex_input::UserIn;
    
    println!("Current result: {:.3}", cur_res);

    loop {
        
        user_input = regex_input::scan_data();
            
        if user_input.operator == 'q' {
            break;
        }
        calc::do_next_op(&mut cur_res, &user_input);
        println!("Current result: {:.3}", cur_res);

    }
    println!("Final result: {:.3}", cur_res);

}
