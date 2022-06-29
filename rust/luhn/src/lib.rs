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

    let doubled_digits_sum: u32 = code
        .chars()
        .rev()
        .skip(1)
        .step_by(2)
        .map(char_to_int)
        .map(transform_digit)
        .sum();

    let other_digits_sum: u32 = code.chars().rev().step_by(2).map(char_to_int).sum();

    let desired_sum = doubled_digits_sum + other_digits_sum; // it was counted twice
    desired_sum % 10 == 0
}
