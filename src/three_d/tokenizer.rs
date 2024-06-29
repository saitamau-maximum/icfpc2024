

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Redirect(Direction), // <, >, ^, v
    Add,                 // +
    Sub,                 // -
    Mul,                 // *
    Div,                 // /
    Mod,                 // %
    Warp,                // @
    Eq,                  // =
    Neq,                 // #
    Submit,              // S
    Alpha,               // A
    Beta,                // B
}

impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Redirect(Direction::Up) => "^".to_string(),
            Operator::Redirect(Direction::Down) => "v".to_string(),
            Operator::Redirect(Direction::Left) => "<".to_string(),
            Operator::Redirect(Direction::Right) => ">".to_string(),
            Operator::Add => "+".to_string(),
            Operator::Sub => "-".to_string(),
            Operator::Mul => "*".to_string(),
            Operator::Div => "/".to_string(),
            Operator::Mod => "%".to_string(),
            Operator::Warp => "@".to_string(),
            Operator::Eq => "=".to_string(),
            Operator::Neq => "#".to_string(),
            Operator::Submit => "S".to_string(),
            Operator::Alpha => "A".to_string(),
            Operator::Beta => "B".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Empty,
    Integer(isize),
    Operator(Operator),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Empty => ".".to_string(),
            Token::Integer(value) => value.to_string(),
            Token::Operator(value) => value.to_string(),
        }
    }
}

pub struct Tokenizer {
    input: Vec<Vec<String>>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        let input = input
            .trim()
            .lines()
            .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
            .collect();
        Tokenizer { input }
    }

    pub fn tokenize(&mut self) -> Vec<Vec<Token>> {
        let mut tokens = vec![];
        for row in self.input.iter() {
            let mut row_tokens = vec![];
            for s in row.iter() {
                if s == "." {
                    row_tokens.push(Token::Empty);
                    continue;
                }
                if let Ok(value) = s.parse::<isize>() {
                    row_tokens.push(Token::Integer(value));
                    continue;
                }
                let op = match s.as_str() {
                    ">" => Operator::Redirect(Direction::Right),
                    "<" => Operator::Redirect(Direction::Left),
                    "^" => Operator::Redirect(Direction::Up),
                    "v" => Operator::Redirect(Direction::Down),
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    "%" => Operator::Mod,
                    "@" => Operator::Warp,
                    "=" => Operator::Eq,
                    "#" => Operator::Neq,
                    "S" => Operator::Submit,
                    "A" => Operator::Alpha,
                    "B" => Operator::Beta,
                    _ => panic!("Invalid operator: {}", s),
                };
                row_tokens.push(Token::Operator(op));
            }
            tokens.push(row_tokens);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let mut tokenizer = Tokenizer::new(
            r#"
. . . . 0 . . . .
. B > . = . . . .
. v 1 . . > . . .
. . - . . . + S .
. . . . . ^ . . .
. . v . . 0 > . .
. . . . . . A + .
. 1 @ 6 . . < . .
. . 3 . 0 @ 3 . .
. . . . . 3 . . .
"#,
        );
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens[0],
            vec![
                Token::Empty,
                Token::Empty,
                Token::Empty,
                Token::Empty,
                Token::Integer(0),
                Token::Empty,
                Token::Empty,
                Token::Empty,
                Token::Empty,
            ]
        );

        assert_eq!(
            tokens[1],
            vec![
                Token::Empty,
                Token::Operator(Operator::Beta),
                Token::Operator(Operator::Redirect(Direction::Right)),
                Token::Empty,
                Token::Operator(Operator::Eq),
                Token::Empty,
                Token::Empty,
                Token::Empty,
                Token::Empty,
            ]
        );
    }
}
