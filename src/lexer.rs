use std::ops::Range;
pub mod tokentype;
pub use tokentype::TokenType;

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
impl<'src> Lexer<'src> {


    pub fn lexer_main(src: &'src str) -> Vec<Token> {
        let mut lexer = Lexer {
            tokens: Vec::new(),
            current: 0,
            source:src,
            strmode:false,
            koedame:String::new(),
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
                                    literal,                     // 肥溜めに貯まっていた文字列が入る
                                    span: start..lexer.current,  // 最初の `"` から 最後の `"` までの範囲
                                });
                            }
                            
                        }
                        _ if lexer.strmode => {
                            lexer.koedame.push(ch);
                        }
                        _ if ch.is_whitespace() => {
                            if (ch == '\t') {
                                lexer.tokens.push(Token {
                                    kind: TokenType::GT_INDENT,
                                    literal:"\t".to_string(),
                                    span:start..lexer.current,
                                });
                            }
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
                                _ => TokenType::GT_IDENT,
                            };
                            lexer.tokens.push(Token {
                                kind,
                                literal: word,
                                span: start..lexer.current,
                            });
                            
                        }
                        _ if ch.is_ascii_digit() => {
                            let mut num = String::new();
                            num.push(ch); // 最初の1文字

                            while let Some(next_ch) = lexer.peek() {
                                if next_ch.is_ascii_digit() {
                                    num.push(lexer.advance().unwrap());
                                } else {
                                    break;
                                }
                            }

                            // 読み終わったらトークンにする
                            lexer.tokens.push(Token {
                                kind: TokenType::GT_INT,
                                literal: num,
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

}