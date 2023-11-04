mod tools;

use getopts::Options;
use rand::rngs::OsRng;
use rand::Rng;
use std::env;

#[allow(dead_code)]
struct Settings {
    length: i32,
    number: bool,
    maj: bool,
    special_char: bool,
}

impl Settings {
    fn new(length: i32, number: bool, maj: bool, special_char: bool) -> Settings {
        Settings {
            length,
            number,
            maj,
            special_char,
        }
    }
    fn display_config(&self) {
        println!(
            "Generating password of len {} with number set as {}, majuscule {} and special character {}.",
            self.length,
            self.number,
            self.maj
            self.special_char
        );
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
fn parser(args: Vec<String>) -> Option<Settings> {
    let mut option = Settings::new(12, false, false, false);
    let mut opts = Options::new();

    opts.optopt(
        "l",
        "length",
        "Define len of password(default 12)",
        "LENGTH",
    );
    opts.optflag("n", "number", "Use number");
    opts.optflag("s", "special", "Use special char");
    opts.optflag("m", "majuscule", "Use majuscule");
    opts.optflag("h", "help", "Display use");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("Error : {}", e);
            return None;
        }
    };

    if matches.opt_present("help") {
        print_usage(&args[0], &opts);
        return None;
    }

    option.length = tools::is_num(&matches.opt_str("length").unwrap_or("12".to_string())).unwrap();

    let option = Settings::new(
        option.length,
        matches.opt_present("number"),
        matches.opt_present("majuscule"),
        matches.opt_present("special"),
    );
    option.display_config();
    Some(option)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = OsRng;

    if let Some(option) = parser(args) {
        let password: String = (0..option.length)
            .map(|_| {
                let nbr = rng.gen_range(32..=126);
                std::char::from_u32(nbr).unwrap()
            })
            .collect();
        println!("{}", password);
    }
}
