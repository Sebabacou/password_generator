mod generator;
mod tools;
mod user_settings;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(option) = user_settings::parser(args) {
        generator::generate_password(option);
    }
}
