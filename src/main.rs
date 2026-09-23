mod lexer;
mod tokentype;

use lexer::Lexer;

fn main() {
    let source_code = "SET X TO 10 (INT).";

    println!("--- レキサーのテスト開始 ---");
    println!("入力ソース: {:?}", source_code);

    match Lexer::lexer_main(source_code) {
        Ok(tokens) => {
            for token in tokens {
                println!(
                    "種類: {:?}, 文字列: {:?}, 範囲: {:?}",
                    token.kind, token.literal, token.span
                );
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