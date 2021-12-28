use {
    super::*,
    std::str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub enum Statement {
    Color(Color),
    MoveTo(Point),
    LineTo(Point),
}

pub enum Token<'s> {
    Word(&'s str),
    Number(f32),
}

fn tokenize(s: &str) -> impl Iterator<Item=Token<'_>> {
    s.split_whitespace()
        .map(|t| {
            match t.parse::<f32>() {
                Ok(n) => Token::Number(n),
                _ => Token::Word(t),
            }
        })
}

impl FromStr for Statement {
    type Err = SyntaxError;
    fn from_str(s: &str) -> Result<Self, SyntaxError> {
        let mut tokens = tokenize(s);
        use {
            SyntaxError::*,
            Token::*,
        };
        macro_rules! end {
            ($statement: expr) => {
                match tokens.next() {
                    Some(_) => Err(ExpectedEnd),
                    None => Ok($statement),
                }
            }
        }
        match tokens.next() {
            Some(Word("color")) => match tokens.next() {
                Some(Word(w)) => match w.parse::<Color>() {
                    Ok(color) => end!(Self::Color(color)),
                    _ => Err(UnknownColor(w.to_string())),
                }
                _ => Err(Other), // TODO
            }
            Some(Word("line")) => match tokens.next() {
                Some(Word("to")) => match (tokens.next(), tokens.next()) {
                    (Some(Number(x)), Some(Number(y))) => end!(Self::LineTo(Point{ x, y })),
                    _ => Err(Other), // TODO
                }
                _ => Err(Other), // TODO
            }
            Some(Word("move")) => match tokens.next() {
                Some(Word("to")) => match (tokens.next(), tokens.next()) {
                    (Some(Number(x)), Some(Number(y))) => end!(Self::MoveTo(Point{ x, y })),
                    _ => Err(Other), // TODO
                }
                _ => Err(Other), // TODO
            }
            Some(Number(n)) => Err(UnexpectedNumber(n)),
            Some(Word(w)) => Err(UnknownVerb(w.to_string())),
            None => Err(Empty),
        }
    }
}
