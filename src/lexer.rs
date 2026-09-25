use crate::tokentype::TokenType;
use std::ops::Range;

pub struct Token {
    pub kind: TokenType,
    pub literal: String,
    pub span: Range<usize>,
}
pub struct Lexer<'src> {
    tokens: Vec<Token>,
    current: usize,
    source: &'src str,
    strmode: bool,
    koedame: String,
}
pub struct LexerError {
    pub message: String,
    pub span: Range<usize>,
}
impl<'src> Lexer<'src> {
    pub fn lexer_main(src: &'src str) -> Result<Vec<Token>, LexerError> {
        
        let mut lexer = Lexer {
            tokens: Vec::new(),
            current: 0,
            source: src,
            strmode: false,
            koedame: String::new(),
        };
        let mut start = 0;
        while !lexer.is_at_end() {
            start = lexer.current;
            match lexer.advance() {
                Some(ch) => {
                    match ch {
                        '"' => {
                            lexer.strmode = !lexer.strmode;

                            if !lexer.strmode {
                                let literal = std::mem::take(&mut lexer.koedame);
                                lexer.tokens.push(Token {
                                    kind: TokenType::GT_STRING,
                                    literal: literal, // 肥溜めに貯まっていた文字列が入る
                                    span: start..lexer.current, // 最初の `"` から 最後の `"` までの範囲
                                });
                            }
                        }
                        _ if lexer.strmode => {
                            lexer.koedame.push(ch);
                        }
                        _ if ch.is_whitespace() => {
                        }
                        _ if ch.is_alphabetic() => {
                            let mut word = String::new();
                            word.push(ch);
                            while let Some(next_ch) = lexer.peek() {
                                if next_ch.is_alphanumeric() {
                                    word.push(lexer.advance().unwrap());
                                } else {
                                    break;
                                }
                            }
                            let kind = match word.as_str() {
                                "SET" => TokenType::GT_SET,
                                "TO" => TokenType::GT_TO,
                                "COMPARE" => TokenType::GT_COMPARE,
                                "DEFINE" => TokenType::GT_DEF,
                                "FUNCTION" => TokenType::GT_FUNC,
                                "NULL" => TokenType::GT_NUL,
                                "INT" => TokenType::GT_SFY_INT,
                                "DISPLAYS" => TokenType::GT_DISPLAYS,
                                "FLOAT" => TokenType::GT_SFY_FLOAT,
                                "STRING" => TokenType::GT_SFY_STRING,
                                "EQUAL" => TokenType::GT_EQ,
                                "NOT" => TokenType::GT_NOT,
                                "PLUS" => TokenType::GT_PLUS,
                                "TIMES" => TokenType::GT_TIMES,
                                "DIV" => TokenType::GT_DIVISION,
                                "MINUS" => TokenType::GT_MINUS,
                                "LOAD" => TokenType::GT_LOAD,
                                "RETURN" => TokenType::GT_RETURN,
                                "COMMENT" => TokenType::GT_COMMENT,
                                "EXIT" => TokenType::GT_EXIT,
                                "OR" => TokenType::GT_OR,
                                "AND" => TokenType::GT_AND,
                                "OVER" => TokenType::GT_OVER,
                                "UNDER" => TokenType::GT_UNDER,
                                "OTHER" => TokenType::GT_OTHER,
                                "END" => TokenType::GT_END,
                                "REPEAT" => TokenType::GT_REPEAT,
                                _ => TokenType::GT_IDENT,
                            };
                            lexer.tokens.push(Token {
                                kind,
                                literal: word,
                                span: start..lexer.current,
                            });
                        }

                        _ if ch.is_ascii_digit() => {
                            let mut float_flag = false;
                            let mut num = String::new();
                            num.push(ch);
                            while let Some(next_ch) = lexer.peek() {
                                if next_ch.is_ascii_digit() || next_ch == '.' {
                                    if next_ch == '.' && num.contains('.') {
                                        return Err(LexerError {
                                            message: "小数点が多すぎます。".to_string(),
                                            span: start..lexer.current,
                                        });
                                    }
                                    if next_ch == '.' {
                                        if lexer.peek_next().is_some_and(|ch| ch.is_ascii_digit()) {
                                            float_flag = true;
                                        } else {
                                            break;
                                        }
                                    }
                                    num.push(lexer.advance().unwrap());
                                } else {
                                    break;
                                }
                            }
                            if num.matches('.').count() == 1 && float_flag == true {
                                lexer.tokens.push(Token {
                                    kind: TokenType::GT_FLOAT,
                                    literal: num,
                                    span: start..lexer.current,
                                });
                            } else {
                                lexer.tokens.push(Token {
                                    kind: TokenType::GT_INT,
                                    literal: num,
                                    span: start..lexer.current,
                                });
                            }
                        }
                        '.' => {
                            lexer.tokens.push(Token {
                                kind: TokenType::GT_PERIOD,
                                literal: ".".to_string(),
                                span: start..lexer.current,
                            });
                        }
                        '(' => {
                            lexer.tokens.push(Token {
                                kind: TokenType::GT_LP,
                                literal: "(".to_string(),
                                span: start..lexer.current,
                            });
                        }
                        ')' => {
                            lexer.tokens.push(Token {
                                kind: TokenType::GT_RP,
                                literal: ")".to_string(),
                                span: start..lexer.current,
                            });
                        }
                        _ => {
                            return Err(LexerError {
                                message: "文法エラー".to_string(),
                                span: start..lexer.current,
                            });
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        return Ok(lexer.tokens);
    }
    pub fn is_at_end(&self) -> bool {
        if (self.current >= self.source.len()) {
            return true;
        } else {
            return false;
        }
    }
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.source[self.current..].chars().next()?;
        self.current += ch.len_utf8();
        Some(ch)
    }
    pub fn peek(&self) -> Option<char> {
        let ch = self.source[self.current..].chars().next()?;
        Some(ch)
    }
    pub fn peek_next(&self) -> Option<char> {
        let ch = self.source[self.current..].chars().nth(1)?;
        Some(ch)
    }
}
