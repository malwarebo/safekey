mod generator;

use std::env;
use generator::generate_password;

fn main() {
    let args: Vec<String> = env::args().collect();

    let length = match args.get(1) {
        Some(arg) => arg.parse::<usize>().unwrap_or(12),
        None => 12,
    };

    let password = generate_password(length);
    println!("{}", password);
}
