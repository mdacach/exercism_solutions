/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.split_whitespace().collect();
    let valid = code.len() > 1 && !code.chars().any(|c| !c.is_digit(10));
    if !valid {
        return false;
    }

    let transform_digit = |d: u32| {
        let double = 2 * d;
        if double >= 10 {
            double - 9
        } else {
            double
        }
    };

    let char_to_int = |c: char| c.to_digit(10).unwrap();

    let digits_sum: u32 = code
        .chars()
        .rev()
        .map(char_to_int)
        .enumerate()
        .map(|(i, d)| if i % 2 == 1 { transform_digit(d) } else { d })
        .sum();

    digits_sum % 10 == 0
}
