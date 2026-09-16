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


    pub fn lexer_main(src: &str) {
        let mut lexer = Lexer {
            tokens: Vec::new(),
            current: 0,
            source:src,
            strmode:false,
            koedame:String::new(),
        };
        let start = 0;
        while !lexer.is_at_end() {
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
                            start = lexer.current;
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
                            while let Some(next_ch) = self.peek() {
                                if next_ch.is_alphanumeric() {
                                    word.push(self.advance().unwrap());
                                } else {
                                    break;
                                }
                                let kind = match word.as_str() {
                                    "SET" => TokenType::SET,
                                    "TO" => TokenType::TO,
                                    _ => TokenType::GT_IDENT,
                                };
                                self.tokens.push(Token {
                                    kind,
                                    literal: word,
                                    span: start..self.current,
                                });
                            }
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
    pub fn peek(&mut self) -> Option<char> {
        let ch = self.source[self.current..].chars().next()?;
        Some(ch)
    }

}