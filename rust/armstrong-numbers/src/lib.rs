pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string().len();

    let mut sum = 0;
    let mut current_value = num;
    while current_value > 0 {
        let last_digit = current_value % 10;
        sum += last_digit.pow(digits as u32);
        current_value /= 10;
    }

    sum == num
}
