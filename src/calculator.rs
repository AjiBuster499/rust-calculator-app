#[derive(Debug, Default)]
pub(crate) struct Calculator {
    pub(crate) equation: String,
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_to_equation(&mut self, c: &str) {
        self.equation.push_str(c);
    }

    pub fn calculate(&self) -> String {
        // TODO This will naively turn a string into an arithmetic equation.
        // In four-function mode, we can guarantee this is safe, because
        // the only characters that can be entered are 0-9, and the four operators.
        // test string: "22+4"
        let mut stack = vec![];
        let split_equation = self.equation.split_whitespace();
        split_equation.rev().for_each(|item| {
            stack.push(item);
        });
        let post_stack = self.postfix(&mut stack);

        String::new()
    }

    fn postfix(&self, stack: &mut Vec<&str>) -> Vec<String> {
        let mut output = vec![];
        while let Some(tmp) = stack.pop() {
            // tmp is the topmost element on our stack
            // need to check if it's a number
            if let Ok(_) = tmp.parse::<i32>() {
                // it's a number
                let tmp = tmp.to_owned() + " ";
                output.push(tmp);
            } else {
                // its not a number
                // so it must be an operator
                match tmp {
                    "+" => {}
                    "-" => {}
                    "*" => {}
                    "/" => {}

                    _ => {}
                }
            }
        }

        output
    }
}
