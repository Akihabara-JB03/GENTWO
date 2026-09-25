use crate::tokentype::TokenType;
use std::ops::Range;
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
    pub span: Range<usize>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
pub enum ParserError {
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
        span: Range<usize>,
        message: String,
    },
    UnexpectedEOF {
        message: String,
    },
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        if token.is_some() {
            self.current += 1;
        }
        token
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    fn check(&self, tokentype: TokenType) -> bool {
        self.peek().map(|token| token.kind) == Some(tokentype)
    }
    fn consume(&mut self, tokentype: TokenType, message: &str) -> Result<Token, ParserError> {
        let token = self.peek().cloned();
        if let Some(token) = token {
            if token.kind == tokentype {
                self.current += 1;
                Ok(token)
            } else {
                Err(ParserError::UnexpectedToken {
                    expected: tokentype,
                    found: token.kind,
                    span: token.span.clone(),
                    message: message.to_string(),
                })
            }
        } else {
            Err(ParserError::UnexpectedEOF {
                message: message.to_string(),
            })
        }
    }
    fn parser_main() {
    
    }
}
