#[derive(Debug, Default)]
pub(crate) struct Calculator {
    /// The Equation container
    equation: String,
    pub display_equation: String,
    pub scientific: bool,
}

impl Calculator {
    /// Generic new function
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_to_equation(&mut self, s: &str) {
        if self.equation.is_empty() {
            self.equation.push_str("( ");
        }
        self.equation.push_str(s);
        self.display_equation.push_str(s);
    }

    /// This will naively turn a string into an arithmetic equation.
    /// In four-function mode, we can guarantee this is safe, because
    /// the only characters that can be entered are 0-9, and the four operators.
    /// test string: "22+4"
    pub fn calculate(&mut self) {
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
            if let Ok(num) = tmp.parse::<f32>() {
                // Test to see if it's readable as a number, and if so,
                // we push that to result.
                numbers.push(num);
            } else {
                // The Err should not be of much concern, because it just means that
                // it's an operator. And it should only be an operator, because we excluded
                // the parentheses in post_stack
                // TODO: Le Unwrap
                let num = numbers.pop().unwrap();
                let sum = match tmp {
                    "+" => numbers.pop().unwrap() + num,
                    "-" => numbers.pop().unwrap() - num,
                    "*" => numbers.pop().unwrap() * num,
                    "/" => numbers.pop().unwrap() / num,
                    // Check that this is guaranteed
                    _ => unreachable!(),
                };
                numbers.push(sum);
            }
        }

        self.equation = numbers.pop().unwrap().to_string();
        self.display_equation = self.equation.clone();
    }

    fn postfix(&self, stack: &mut Vec<&str>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let mut operators = vec![];
        while let Some(tmp) = stack.pop() {
            // tmp is the topmost element on our stack
            // need to check if it's a number
            if tmp.parse::<f32>().is_ok() {
                // it's a number
                output.push(tmp.to_string());
            } else {
                // its not a number
                match tmp {
                    "(" => {
                        operators.push(tmp);
                    }
                    ")" => {
                        let mut operator = operators.pop();
                        while let Some(op) = operator {
                            if op == "(" {
                                break;
                            }
                            output.push(op.to_owned() + " ");
                            operator = operators.pop();
                        }
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
                    }
                }
            }
        }

        output.reverse();
        output
    }

    fn pemdas(&self, op: String) -> i8 {
        // TODO: Add in Scientific functions
        // (log, ln, sin, exp)
        match op.as_str() {
            "^" => 1,
            "+" | "-" => 2,
            "*" | "/" => 3,
            _ => -1,
        }
    }
    pub fn clear(&mut self) {
        self.equation.clear();
        self.display_equation.clear();
    }
}

// Tests for the Calculator struct
#[cfg(test)]
mod tests {

    use super::Calculator;

    #[test]
    fn test_calculate_simple() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("22 + 4");
        mock_calc.push_to_equation(" )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "26");
    }

    #[test]
    fn test_calculate_complex() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("22 + 4 / 5 - 3");
        mock_calc.push_to_equation(" )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "19.8");
    }

    #[test]
    fn test_paren_prefix() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("4 ( 5 + 7 ) )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "48");
    }
}
