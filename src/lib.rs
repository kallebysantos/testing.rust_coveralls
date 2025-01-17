pub fn is_even(value: u8) -> bool {
    if value % 2 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2))
    }
}
