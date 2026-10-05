mod lexer;
mod tokentype;
mod parse;

use lexer::Lexer;
use parse::Parser;

fn main() {
    let source_code = "3 PLUS X PLUS 3";

    println!("--- レキサーのテスト開始 ---");
    println!("入力ソース: {:?}", source_code);

    match Lexer::lexer_main(source_code) {
        Ok(tokens) => {
            for token in &tokens {
                println!(
                    "種類: {:?}, 文字列: {:?}, 範囲: {:?}",
                    token.kind, token.literal, token.span
                );
            }

            let mut parser = Parser {
                tokens: &tokens,
                current: 0,
            };

            match parser.expression() {
                Ok(expr) => {
                    println!("Expression: {:?}", expr);
                }
                Err(error) => {
                    println!("Parser Error: {:?}", error);
                }
            }
        }

        Err(error) => {
            println!(
                "Lexer Error: {} (範囲: {:?})",
                error.message, error.span
            );
        }
    }

    println!("--- テスト終了 ---");
}
