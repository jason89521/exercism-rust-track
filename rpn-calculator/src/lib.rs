#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn calculate(a: i32, b: i32, op: &CalculatorInput) -> Option<i32> {
    match op {
        CalculatorInput::Add => Some(a + b),
        CalculatorInput::Divide => Some(a / b),
        CalculatorInput::Multiply => Some(a * b),
        CalculatorInput::Subtract => Some(a - b),
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for op in inputs {
        if let CalculatorInput::Value(value) = op {
            stack.push(*value)
        } else {
            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                match calculate(a, b, op) {
                    Some(value) => stack.push(value),
                    None => return None,
                }
            } else {
                return None;
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
