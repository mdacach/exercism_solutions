#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn pop_two(stack: &mut Vec<i32>) -> Option<(i32, i32)> {
    if stack.len() < 2 {
        return None;
    }

    Some((stack.pop().unwrap(), stack.pop().unwrap()))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for symbol in inputs {
        match symbol {
            CalculatorInput::Value(v) => stack.push(*v),
            CalculatorInput::Add => {
                if let Some((rhs, lhs)) = pop_two(&mut stack) {
                    stack.push(lhs + rhs);
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if let Some((rhs, lhs)) = pop_two(&mut stack) {
                    stack.push(lhs - rhs);
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if let Some((rhs, lhs)) = pop_two(&mut stack) {
                    stack.push(lhs * rhs);
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if let Some((rhs, lhs)) = pop_two(&mut stack) {
                    stack.push(lhs / rhs);
                } else {
                    return None;
                }
            }
        }
    }

    if stack.len() == 1 {
        return Some(stack.pop().unwrap());
    }

    None
}
