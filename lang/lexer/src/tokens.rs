#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Whitespace,
    Identifier(String),
    Litteral(LitteralKind),
    InvalidToken,
    EOF,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LitteralKind {
    String(String),
    Boolean(String),
    Integer(String),
}
