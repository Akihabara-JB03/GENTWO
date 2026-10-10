use crate::tokentype::TokenType;
use std::ops::Range;
#[derive(Clone)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
    pub span: Range<usize>,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}
pub enum ParserError {
    UnexpectedToken {
        expected: Vec<TokenType>,
        found: TokenType,
        span: Range<usize>,
        message: String,
    },
    UnexpectedEOF {
        message: String,
    },
    Int64ExchangeError {
        literal: String,
        span: Range<usize>,
    },
    Float64ExchangeError {
        literal: String,
        span: Range<usize>,
    },
}
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Unary {
        operator:Operator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Equal,
}
pub enum Statement {
    Set {
        name: String,
        value: Expr,
    },
    Displays {
        value: Expr,
    },
    Exit,
    End,
}
pub enum Operator {
    Add,
    Div,
    Times,
    Minus,
    And,
    Or,
    Equal,
    Not,
    Under,
    Over,
}
pub enum WarningKind {
    ImplicitConversion,
    UnusedVariable,
}

pub struct Warning {
    pub kind: WarningKind,
    pub message: String,
}
pub struct Ast {
    Statements: Vec<Statement>,
}
pub struct PR { /*Parser Result*/
    pub ast: Ast,
    pub warnings: Vec<Warning>,
}

impl<'a> Parser<'a> {
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
                    expected: vec![tokentype],
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
    fn parser_main(tokens: &[Token]) -> Result<PR,ParserError> {
        let mut parser = Parser {
            tokens,
            current: 0,
        };
        parser.parse()

    }
    fn parse(&mut self) -> Result<PR,ParserError> {
        let mut statements = Vec::new();
        let mut warnings = Vec::new();
        while !self.is_at_end() {

        }
    }
    fn statement(&mut self) -> Result<Statement, ParserError> {
        // 次のTokenを見て、どのStatementなのか判断
        match self.peek() {
            TokenType::GT_SET => self.parser_set()
        }
    }
    fn parse_set(&mut self) -> Result<Statement, ParserError> {
        self.consume(TokenType::GT_SET, "")?;
        let name = self.consume(TokenType::GT_IDENT)?;
        self.consume(TokenType::GT_TO)?;
        let val = self.consume(TokenType::GT_INT)?;
        self.consume(TokenType::GT_PERIOD)?;
        Ok(Statement {
            name:name.literal,
            val.literal,
        })
    }
    fn expression(&mut self) -> Result<Expr,ParserError> {
        self.factor()
    }
    fn parse_equal(&mut self) -> Result<Expr, ParserError> {
        self.consume(TokenType::GT_EQ, "EQUALが必要です。")?;
        Ok(Expr::Equal)
    }
    fn eof_check(&self) -> Result<&Token, ParserError> {
        match self.peek() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF {
                message: "式を期待しました。".to_string(),
            }),
        }
    }
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        while self.peek().map(|token| token.kind) == Some(TokenType::GT_PLUS) || self.peek().map(|token| token.kind) == Some(TokenType::GT_MINUS) {
            match self.peek().map(|token| token.kind) {
                Some(TokenType::GT_PLUS) => {
                    self.advance();
                    let right = self.term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: Operator::Add,
                        right: Box::new(right),
                    };
                }
                Some(TokenType::GT_MINUS) => {
                    self.advance();
                    let right = self.term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: Operator::Minus,
                        right: Box::new(right),
                    };
                }
                _ => {}
            }
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        while self.peek().map(|token| token.kind) == Some(TokenType::GT_TIMES) || self.peek().map(|token| token.kind) == Some(TokenType::GT_DIVISION) {
            match self.peek().map(|token| token.kind) {
                Some(TokenType::GT_TIMES) => {
                    self.advance();
                    let right = self.unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: Operator::Times,
                        right: Box::new(right),
                    };
                }
                Some(TokenType::GT_DIVISION) => {
                    self.advance();
                    let right = self.unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: Operator::Div,
                        right: Box::new(right),
                    };
                }
                _ => {}
            }
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().map(|token| token.kind) {
            Some(TokenType::GT_MINUS) => {
                self.advance();
                let right = self.unary()?;
                Ok(Expr::Unary {
                    operator:Operator::Minus,
                    right: Box::new(right),
                })
            }
            Some(TokenType::GT_NOT) => {
                self.advance();
                let right = self.parse_equal()?;
                Ok(Expr::Unary {
                    operator: Operator::Not,
                    right: Box::new(right),
                })
            }
            _ => {
                self.primary()
            }
        }
    }
    fn primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.eof_check()?;
        match token.kind {
            TokenType::GT_INT => {
                // 整数
                let token = self.advance().unwrap();

                let value = token.literal.parse::<i64>()
                    .map_err(|_| ParserError::Int64ExchangeError {
                        literal: token.literal.clone(),
                        span: token.span.clone(),
                    })?;
                Ok(Expr::Integer(value))
            }

            TokenType::GT_FLOAT => {
                // 浮動小数点数
                let token = self.advance().unwrap();
                let val = token.literal.parse::<f64>()
                    .map_err(|_| ParserError::Float64ExchangeError {
                        literal: token.literal.clone(),
                        span: token.span.clone(),
                    })?;
                Ok(Expr::Float(val))
            }

            TokenType::GT_STRING => {
                let token = self.advance().unwrap();
                Ok(Expr::String(token.literal))
            }

            TokenType::GT_IDENT => {
                let token = self.advance().unwrap();
                Ok(Expr::Identifier(token.literal))
            }

            TokenType::GT_LP => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::GT_RP)?;
                Ok(expr)
            }

            _ => {
                // UnexpectedToken
                Err(ParserError::UnexpectedToken {
                    expected: vec![
                    TokenType::GT_INT,
                    TokenType::GT_FLOAT,
                    TokenType::GT_STRING,
                    TokenType::GT_IDENT,
                    TokenType::GT_LP
                    ],
                    found: token.kind,
                    span: token.span.clone(),
                    message: "式として解釈できないTokenです。".to_string(),
                })
            }
        }
    }
}