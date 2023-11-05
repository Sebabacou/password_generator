use crate::user_settings::Settings;
use rand::rngs::OsRng;
use rand::Rng;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum CharacterCategory {
    Uppercase,
    Lowercase,
    Digit,
    Special,
}

fn get_value_categories(character_category: CharacterCategory) -> &'static str{
    match character_category {
        CharacterCategory::Lowercase => "abcdefghijklmnopqrstuvwxyz",
        CharacterCategory::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        CharacterCategory::Digit => "0123456789",
        CharacterCategory::Special => "!#$%&'()*+,-./:;<=>?@[\\]^_{|}~\""
    }
}

pub fn generate_password(option: Settings) {
    let mut rng = OsRng;
    let mut possible_char = String::from(get_value_categories(CharacterCategory::Lowercase));

    let mut categories_to_include = HashMap::new();
    categories_to_include.insert(CharacterCategory::Digit, option.digit);
    categories_to_include.insert(CharacterCategory::Uppercase, option.uppercase);
    categories_to_include.insert(CharacterCategory::Special, option.special_char);

    for (value, include) in categories_to_include.iter() {
        if *include {
            possible_char.push_str(get_value_categories(*value));
        }
    }

    let password: String = (0..option.length)
        .fold(String::new(), |mut char, _| {
            let random_char = possible_char
                .chars()
                .nth(rng.gen_range(0..possible_char.len()))
                .unwrap_or('a');
            char.push(random_char);
            char
        });
    println!("{}", password);
}