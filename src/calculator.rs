#[derive(Debug, Default)]
pub(crate) struct Calculator {
    pub(crate) equation: String,
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_to_equation(&mut self, s: &str) {
        if self.equation.is_empty() {
            self.equation.push_str("( ");
        }
        self.equation.push_str(s);
    }

    pub fn calculate(&mut self) -> String {
        // TODO This will naively turn a string into an arithmetic equation.
        // In four-function mode, we can guarantee this is safe, because
        // the only characters that can be entered are 0-9, and the four operators.
        // test string: "22+4"
        let mut stack = vec![];
        let split_equation = self.equation.split_whitespace();
        split_equation.rev().for_each(|item| {
            stack.push(item);
        });
        // Converts the equation into a postfix configuration (i.e. 22 4 +)
        let mut post_stack = self.postfix(&mut stack);

        // We will be doing Reverse Polish Notation because it's basically
        // built for stacks.
        let mut numbers = vec![];
        while let Some(tmp) = post_stack.pop() {
            // Remove that pesky whitespace
            let tmp = tmp.trim();
            if let Ok(num) = tmp.parse::<i32>() {
                // Test to see if it's readable as a number, and if so,
                // we push that to result.
                numbers.push(num);
            } else {
                // The Err should not be of much concern, because it just means that
                // it's an operator. And it should only be an operator, because we excluded
                // the parentheses in post_stack
                let sum;
                // TODO: Le Unwrap
                let num = numbers.pop().unwrap();
                match tmp {
                    "+" => {
                        sum = numbers.pop().unwrap() + num;
                    }
                    "-" => {
                        sum = numbers.pop().unwrap() - num;
                    }
                    "*" => {
                        sum = numbers.pop().unwrap() * num;
                    }
                    "/" => {
                        sum = numbers.pop().unwrap() / num;
                    }
                    // Check that this is guaranteed
                    _ => unreachable!(),
                }
                numbers.push(sum);
            }
        }

        self.equation.clear();
        numbers.pop().unwrap().to_string()
    }

    // TODO: Test Cases for this
    fn postfix(&self, stack: &mut Vec<&str>) -> Vec<String> {
        let mut end_of_scope = false;
        let mut output = vec![];
        let mut operators = vec![];
        while let Some(tmp) = stack.pop() {
            // tmp is the topmost element on our stack
            // need to check if it's a number
            if let Ok(_) = tmp.parse::<i32>() {
                // it's a number
                // the "( " is added to denote the start of an operator scope
                let tmp = "( ".to_owned() + tmp + " ";
                output.push(tmp);
                if end_of_scope {
                    // This is appended at the end of an operator scope
                    output.push(" )".to_string());
                    end_of_scope = false;
                }
            } else {
                // its not a number
                match tmp {
                    // Start of a scope
                    "(" => {
                        operators.push(tmp);
                    }
                    ")" => {
                        // TODO: There should be a better way to do this
                        let mut operator = operators.pop();
                        while let Some(op) = operator {
                            if op == "(" {
                                break;
                            }
                            output.push(op.to_owned() + " ");
                            output.push(op.to_owned() + " ");
                            operator = operators.pop();
                        }
                        // End of a scope
                    }
                    _ => {
                        // These are the operators
                        // TODO: Le Unwraps
                        while !operators.is_empty()
                            && self.pemdas(operators.last().unwrap().to_string())
                                >= self.pemdas(tmp.to_owned())
                        {
                            output.push(operators.pop().unwrap().to_string() + " ");
                        }
                        operators.push(tmp);
                        end_of_scope = true;
                    }
                }
            }
        }

        output.reverse();
        output
    }

    fn pemdas(&self, op: String) -> i8 {
        match op.as_str() {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => -1,
        }
    }
}

// Tests for the Calculator struct
#[cfg(test)]
mod tests {
    // TODO: Massive Tests
    // TODO: Need to test postfix and calculate

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
