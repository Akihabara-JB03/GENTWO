mod lexer;
use lexer::Lexer;
mod tokentype;
fn main() {
    // あなたが作った小数判別ロジックが正しく動くか試すためのテストケース
    // 整数、小数、そしてドットが連続するケース
    let source_code = "123 3.14 1..5";

    println!("--- レキサーのテスト開始 ---");
    println!("入力ソース: {:?}", source_code);

    // あなたのレキサーを呼び出す
    let tokens = Lexer::lexer_main(source_code);

    // 切り出されたトークンを1つずつ画面に表示
    for token in tokens {
        // ※TokenType や Token 構造体に Debug 属性（#[derive(Debug)]）がついている前提です
        println!(
            "種類: {:?}, 文字列: {:?}, 範囲: {:?}",
            token.kind, token.literal, token.span
        );
    }
    println!("--- テスト終了 ---");
}
