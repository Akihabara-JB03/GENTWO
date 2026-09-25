mod lexer;
mod tokentype;
mod parse;
use lexer::Lexer;

fn main() {
    let source_code = "REPEAT 10 TIMES
        SET X TO X PLUS 1.
        DISPLAYS X.
    END.";
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