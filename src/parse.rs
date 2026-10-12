/*
    ____________________________                                                            
   |                            |      ________________________________                     
   | File Created: Sep 14, 2026 |_____|      GENTWO     |_____________/\                    
   |       Akihabara-JB03       |     |_________________|     \      /  \         [ Parser ]
   |____________________________| ____________________________/\    /    \            /     
                | |             | Last Modified: Oct 12, 2026 | \  /      \          /      
                | |____________/| Last Commit: Oct 10, 2026   |  \/        \________/       
                |______________/|_____________________________|                             
*/
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
    Null,
    Unary {
        operator:Operator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
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
    Compare {
        value:Expr,
    },
    Load {
        module:Token,
    },
}
pub enum Operator {
    Add,
    Div,
    Times,
    Minus,
    And,
    Or,
    NotEqual,
    Under,
    Over,
    UnderOrEqual,
    OverOrEqual,
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
    pub fn parse(&mut self) -> Result<PR,ParserError> {
        let mut statements = Vec::new();
        let mut warnings = Vec::new();
        while !self.is_at_end() {

        }
    }
    fn statement(&mut self) -> Result<Statement, ParserError> {
        let token = self.eof_check()?;
        match token.kind {
            TokenType::GT_SET => self.parse_set(),
            TokenType::GT_COMPARE => self.spc(),
            TokenType::GT_DISPLAYS => parse_displays(),
            TokenType::GT_LOAD => parse_load(),
        }
    }
    fn parse_displays(&mut self) -> Result<Statement,ParserError> {
        self.consume(TokenType::GT_DISPLAYS, "エラー　PRINTで文字を表示するって！！印刷ぅ！？心底驚きだわｗｗｗ")?;
        let value = self.expression()?;
        self.consume(TokenType::GT_PERIOD, "文の終わりはピ・リ・オ・ド！！")?;
        Ok(Statement::Displays {
            value
        })
    }
    fn parse_load(&mut self) -> Result<Statement, ParserError> {
        self.consume(TokenType::GT_LOAD, "LOADじゃなくてLoadなんだよね。ごめん。大文字小文字で違うとエラーなんだ。")?;
        let module = self.consume(
            TokenType::GT_IDENT,
            "エラー　読み込むモジュール名を指定してくれ！！"
        )?;
        self.consume(TokenType::GT_PERIOD, "文の終わりはピ・リ・オ・ド！！")?;
        Ok(Statement::Load {
            module
        })
    }
    fn parse_set(&mut self) -> Result<Statement, ParserError> {
        self.consume(TokenType::GT_SET, "エラー　変数の設定は、SET文に通ずる")?;
        let name = self.consume(TokenType::GT_IDENT, "エラー　変数名を忘れるな！！何に何を入れるんだよ！！")?;
        self.consume(TokenType::GT_TO, "エラー　英語の読みやすさのためのTOを消すのは腹切りされてもおかしくない。")?;
        let val = self.expression()?;
        self.consume(TokenType::GT_PERIOD, "文の終わりはピ・リ・オ・ド！！")?;
        Ok(Statement::Set {
            name:name.literal,
            value:val,
        })
    }
    fn parse_compare(&mut self) -> Result<Expr, ParserError> {
        self.consume(TokenType::GT_COMPARE, "エラー　比較の始まりはIfじゃない！！！それはライバル！！")?;
        let src = self.expression()?;
        self.consume(TokenType::GT_TO, "エラー　英語の読みやすさのためのTOを消すのは腹切りされてもおかしくない。")?;
        let dst = self.expression()?;
        self.consume(TokenType::GT_LP, "エラー　演算子はカッコで囲む必要があります。")?;
        let operator = self.parse_equal()?;
        self.consume(TokenType::GT_RP, "エラー　演算子はカッコで囲む必要があります。")?;
        Ok(Expr::Binary {
            left: Box::new(src),
            operator,
            right: Box::new(dst),
        })
    }
    fn spc(&mut self) /*Statement Parse Compare*/ -> Result<Statement, ParserError> {
        let value = self.parse_compare()?;
        self.consume(TokenType::GT_PERIOD, "文の終わりはピ・リ・オ・ド！！")?;
        Ok(Statement::Compare {
            value
        })
    }
    fn expression(&mut self) -> Result<Expr,ParserError> {
        self.or()
    }
    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.peek().map(|token| token.kind) == Some(TokenType::GT_OR) {
            self.advance();
            let right = self.and()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Operator::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        while self.peek().map(|token| token.kind) == Some(TokenType::GT_AND) {
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: Operator::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn parse_equal(&mut self) -> Result<Operator, ParserError> {
        if self.peek().map(|token| token.kind) == Some(TokenType::GT_NOT) {
            self.advance();
            self.consume(TokenType::GT_EQ, "EQUALが必要です。")?;
            return Ok(Operator::NotEqual);
        }
        if self.peek().map(|token| token.kind) == Some(TokenType::GT_UNDER) {
            self.advance();
            if self.peek().map(|token| token.kind) == Some(TokenType::GT_OR) {
                self.advance();
                self.consume(TokenType::GT_EQ, "EQUALが必要です。")?;
                return Ok(Operator::UnderOrEqual)
            }
            return Ok(Operator::Under);
        }
        if self.peek().map(|token| token.kind) == Some(TokenType::GT_OVER) {
            self.advance();
            if self.peek().map(|token| token.kind) == Some(TokenType::GT_OR) {
                self.advance();
                self.consume(TokenType::GT_EQ, "EQUALが必要です。")?;
                return Ok(Operator::OverOrEqual)
            }
            return Ok(Operator::Over);
        }
        self.consume(TokenType::GT_EQ, "EQUALが必要です。")?;
        Ok(Operator::Equal)
    }
    fn eof_check(&self) -> Result<&Token, ParserError> {
        match self.peek() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF {
                message: "式を期待しました。".to_string(),
            }),
        }
    }
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        while self.peek().map(|token| token.kind) == Some(TokenType::GT_PLUS) || self.peek().map(|token| token.kind) == Some(TokenType::GT_MINUS) {
            match self.peek().map(|token| token.kind) {
                Some(TokenType::GT_PLUS) => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: Operator::Add,
                        right: Box::new(right),
                    };
                }
                Some(TokenType::GT_MINUS) => {
                    self.advance();
                    let right = self.factor()?;
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
    fn factor(&mut self) -> Result<Expr, ParserError> {
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
                Ok(Expr::String(token.literal.clone()))
            }

            TokenType::GT_IDENT => {
                let token = self.advance().unwrap();
                Ok(Expr::Identifier(token.literal.clone()))
            }

            TokenType::GT_LP => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::GT_RP,  "閉じ括弧 ')' が必要です。")?;
                Ok(expr)
            }
            TokenType::GT_NUL => {
                Ok(Expr::Null)
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