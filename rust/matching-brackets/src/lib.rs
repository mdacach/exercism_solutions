pub fn brackets_are_balanced(string: &str) -> bool {
    let mut last_opened = Vec::new();
    for ch in string.chars() {
        match ch {
            '[' => last_opened.push('['),
            '{' => last_opened.push('{'),
            '(' => last_opened.push('('),
            ']' => {
                if last_opened.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if last_opened.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if last_opened.pop() != Some('(') {
                    return false;
                }
            }
            _ => {}
        };
    }

    last_opened.is_empty()
}
