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
                let (rhs, lhs) = pop_two(&mut stack)?;
                stack.push(lhs + rhs);
            }
            CalculatorInput::Subtract => {
                let (rhs, lhs) = pop_two(&mut stack)?;
                stack.push(lhs - rhs);
            }
            CalculatorInput::Multiply => {
                let (rhs, lhs) = pop_two(&mut stack)?;
                stack.push(lhs * rhs);
            }
            CalculatorInput::Divide => {
                let (rhs, lhs) = pop_two(&mut stack)?;
                stack.push(lhs / rhs);
            }
        }
    }

    if stack.len() == 1 {
        return stack.pop();
    }

    None
}
