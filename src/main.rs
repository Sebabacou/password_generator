mod tools;

use getopts::Options;
use rand::rngs::OsRng;
use rand::Rng;
use std::env;

#[allow(dead_code)]
struct Settings {
    length: i32,
    digit: bool,
    uppercase: bool,
    special_char: bool,
}

impl Settings {
    fn new(length: i32, digit: bool, uppercase: bool, special_char: bool) -> Settings {
        Settings {
            length,
            digit,
            uppercase,
            special_char,
        }
    }
    fn display_config(&self) {
        println!(
            "Generating password of len {} with digit set as {}, uppercase {} and special character {}.",
            self.length,
            self.digit,
            self.uppercase,
            self.special_char
        );
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
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
    opts.optflag("d", "digit", "Use digit");
    opts.optflag("s", "special", "Use special char");
    opts.optflag("u", "uppercase", "Use uppercase");
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
        matches.opt_present("digit"),
        matches.opt_present("uppercase"),
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
