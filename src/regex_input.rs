pub fn scan_data() -> UserIn {

    let mut input = String::new();

    let re = regex::Regex::new(r"(?<operator>.)s*(?<operand>[0-9]+\.?[0-9]*)").unwrap();
    
}