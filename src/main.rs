mod roll;

use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let result = roll::roll::execute(&arguments);
    print!("Total: {}\n\n", result);
}
