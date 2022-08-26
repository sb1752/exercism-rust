use CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            Value(num) => stack.push(*num),
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();

                match input {
                    Add => stack.push(num2 + num1),
                    Subtract => stack.push(num2 - num1),
                    Multiply => stack.push(num2 * num1),
                    Divide => stack.push(num2 / num1),
                    _ => return None,
                }
            }
        }
    }

    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
