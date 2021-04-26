mod roll;
mod input_element;

use std::env;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    roll::parser::parse(&arguments);
}
