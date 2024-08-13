pub fn is_num(value: &str) -> Option<i32> {
    match value.parse::<i32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
