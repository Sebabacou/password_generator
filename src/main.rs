mod generator;
mod user_settings;
mod tools;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(option) = user_settings::parser(args) {
        generator::generate_password(option);
    }
}
