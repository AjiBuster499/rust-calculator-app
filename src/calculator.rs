#[derive(Debug, Default)]
pub(crate) struct Calculator {
    /// The actual equation
    equation: String,
    /// What is displayed on the calculator, used to hide the parentheses that are at the beginning
    /// to open the global scope.
    pub display_equation: String,
    /// Flag to display the scientific functions
    pub scientific: bool,
}

impl Calculator {
    /// Generic new function
    pub fn new() -> Self {
        Self::default()
    }

    /// Add some string to the equation.
    pub fn push_to_equation(&mut self, s: &str) {
        if self.equation.is_empty() {
            // Opens the global scope.
            // TODO: See if this can be eliminated.
            self.equation.push_str("( ");
        }
        self.equation.push_str(s);
        self.display_equation.push_str(s);
    }

    /// Parses self.equation and calculates the answer.
    /// Calls self.postfix() to convert the equation into a postfix operator stack.
    /// The result is then cloned to self.display_equation. The result is also kept inside self.equation.
    pub fn calculate(&mut self) {
        let mut stack = vec![];
        let split_equation = self.equation.split_whitespace();
        split_equation.rev().for_each(|item| {
            stack.push(item);
        });

        // Converts the equation into a postfix configuration, so that 22 + 4 becomes 22 4 +
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
                    "log" => f32::log10(num),
                    "ln" => f32::ln(num),
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

    /// Calculates order of operations, following PEMDAS:
    ///     1. Parenthesis
    ///     2. Exponents
    ///     3. Multiplication/Division
    ///     4. Addition/Subtraction
    fn pemdas(&self, op: String) -> i8 {
        // TODO: Add in Scientific functions
        // (log, ln, sin, exp)
        match op.as_str() {
            "^" | "log" | "ln" => 1,
            "+" | "-" => 2,
            "*" | "/" => 3,
            _ => -1,
        }
    }

    /// Clears the equation, resetting the calculator
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
        mock_calc.push_to_equation("22 + 4 )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "26");
    }

    #[test]
    fn test_calculate_complex() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("22 + 4 / 5 - 3 )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "19.8");
    }

    #[test]
    #[should_panic]
    /// Currently this is expected to fail, as an open parenthesis against some number does not
    /// equate to multiplication by default. This may be added in the future, hence this test
    /// remains.
    fn test_paren_prefix() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("4 ( 5 + 7 ) )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "48");
    }

    #[test]
    fn test_logarithm() {
        let mut mock_calc = Calculator::new();
        mock_calc.push_to_equation("log ( 100 ) )");
        mock_calc.calculate();
        assert_eq!(mock_calc.display_equation, "2");
    }

    #[test]
    fn test_natural_logarithm() {}
}
