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
            _ => {
                let (rhs, lhs) = pop_two(&mut stack)?;
                match symbol {
                    CalculatorInput::Add => stack.push(lhs + rhs),
                    CalculatorInput::Subtract => stack.push(lhs - rhs),
                    CalculatorInput::Multiply => stack.push(lhs * rhs),
                    CalculatorInput::Divide => stack.push(lhs / rhs),
                    _ => (),
                }
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop();
    }

    None
}
