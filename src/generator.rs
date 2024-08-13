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

fn get_value_categories(character_category: CharacterCategory) -> &'static str {
    match character_category {
        CharacterCategory::Lowercase => "abcdefghijklmnopqrstuvwxyz",
        CharacterCategory::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        CharacterCategory::Digit => "0123456789",
        CharacterCategory::Special => "!#$%&'()*+,-./:;<=>?@[\\]^_{|}~\"",
    }
}

pub fn generate_password(option: Settings) -> String {
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

    let password: String = (0..option.length).fold(String::new(), |mut char, _| {
        let random_char = possible_char
            .chars()
            .nth(rng.gen_range(0..possible_char.len()))
            .unwrap_or('a');
        char.push(random_char);
        char
    });
    println!("{}", password);
    password
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_settings::Settings;

    #[test]
    fn generates_password_of_correct_length() {
        let settings = Settings {
            length: 10,
            digit: true,
            uppercase: true,
            special_char: true,
        };
        let password = generate_password(settings);
        assert_eq!(password.len(), 10);
    }
    #[test]
    fn generates_password_with_only_lowercase_when_no_other_categories_selected() {
        let settings = Settings {
            length: 10,
            digit: false,
            uppercase: false,
            special_char: false,
        };
        let password = generate_password(settings);
        assert!(password.chars().all(|c| c.is_lowercase()));
    }

    #[test]
    fn generates_password_with_uppercase_when_selected() {
        let settings = Settings {
            length: 10,
            digit: false,
            uppercase: true,
            special_char: false,
        };
        let password = generate_password(settings);
        assert!(password.chars().any(|c| c.is_uppercase()));
    }

    #[test]
    fn generates_password_with_digit_when_selected() {
        let settings = Settings {
            length: 10,
            digit: true,
            uppercase: false,
            special_char: false,
        };
        let password = generate_password(settings);
        assert!(password.chars().any(|c| c.is_numeric()));
    }

    #[test]
    fn generates_password_with_special_char_when_selected() {
        let settings = Settings {
            length: 10,
            digit: false,
            uppercase: false,
            special_char: true,
        };
        let password = generate_password(settings);
        let special_chars = "!#$%&'()*+,-./:;<=>?@[\\]^_{|}~\"";
        assert!(password.chars().any(|c| special_chars.contains(c)));
    }
}
