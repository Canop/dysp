use {
    std::fmt,
};

#[derive(Debug)]
pub enum SyntaxError {
    Empty,
    UnknownVerb(String),
    UnexpectedWord(String),
    UnexpectedNumber(f32),
    ExpectedEnd,
    Other,
    UnknownColor(String),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownVerb(verb) => {
                write!(f, "Unknown verb: {:?}", verb)
            }
            Self::UnexpectedWord(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
            Self::UnexpectedNumber(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
            Self::ExpectedEnd => {
                write!(f, "Unexpected continuation")
            }
            Self::Empty => {
                write!(f, "Empty")
            }
            Self::Other => {
                write!(f, "Other")
            }
            Self::UnknownColor(token) => {
                write!(f, "Unknown color: {:?}", token)
            }
        }
    }
}
