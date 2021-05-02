mod roll;
mod input_element;

use std::env;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let result = roll::parser::parse(&arguments);
    print!("Total: {}\n\n", result);
}
