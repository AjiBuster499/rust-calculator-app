pub(crate) enum Operators {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

impl From<&str> for Operators {
    fn from(s: &str) -> Self {
        return match s {
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "/" => Self::Division,
            "*" => Self::Multiplication,
            // TODO: Figure out if this should be unreachable or not
            _ => unreachable!(),
        };
    }
}
