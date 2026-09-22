pub mod tokentype;
pub use tokentype::TokenType;
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
