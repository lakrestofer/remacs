use std::{iter::Peekable, str::Chars};

mod tokens;

const OPERATOR_CHARS: &str = r#"+/"#;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    has_reached_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            has_reached_eof: false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = tokens::Token;

    fn next(&mut self) -> Option<Self::Item> {
        use tokens::Token;

        if self.has_reached_eof {
            return None;
        }

        match self.chars.next() {
            Some('(') => Some(Token::OpenParen),
            Some(')') => Some(Token::CloseParen),
            Some(c) if c.is_whitespace() => Some(Token::Whitespace),
            Some(c) if is_valid_ident(c) => {
                let mut ident = String::from(c);

                while self.chars.peek().is_some() && is_valid_ident(*self.chars.peek().unwrap()) {
                    let next = self.chars.next();
                    ident.push(next.unwrap())
                }

                Some(Token::Identifier(ident))
            }

            None => {
                self.has_reached_eof = true;
                Some(Token::EOF)
            }
            _ => Some(Token::InvalidToken),
        }
    }
}

fn is_operator(c: char) -> bool {
    OPERATOR_CHARS.contains(c)
}

fn is_valid_ident(c: char) -> bool {
    !c.is_whitespace() && (is_operator(c) || c.is_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::{
        tokens::{Token, Token::*},
        *,
    };

    fn is_eq(expected: &[Token], actual: &[Token]) {
        if expected.len() != actual.len() {
            panic!(
                "expected len is not equal to actual len. expected: {}, actual: {}",
                expected.len(),
                actual.len()
            );
        }

        for (first, second) in expected.iter().zip(actual.iter()) {
            assert_eq!(*first, *second);
        }
    }

    #[test]
    fn parens() {
        let input = "(((((())))))";
        let lexer = Lexer::new(input);
        let expected = &[
            OpenParen, OpenParen, OpenParen, OpenParen, OpenParen, OpenParen, CloseParen,
            CloseParen, CloseParen, CloseParen, CloseParen, CloseParen, EOF,
        ];
        let actual: Vec<Token> = lexer.collect();
        is_eq(expected, &actual);
    }

    #[test]
    fn ident() {
        let input = "(+ 1 1)";
        let lexer = Lexer::new(input);
        let expected = &[
            OpenParen,
            Identifier("+".into()),
            Identifier("1".into()),
            Identifier("1".into()),
            CloseParen,
            EOF,
        ];
        let actual: Vec<Token> = lexer.filter(|t| *t != Whitespace).collect();
        println!("Expected: {:?}", expected);
        println!("Actual: {:?}", actual);
        is_eq(expected, &actual);
    }
}
