use byte_asm::lex::{Lexer, TokenType};

fn main() {
    let file_path = std::env::args().nth(1).unwrap_or("test.s".to_string());
    let mut lexer =
        Lexer::new(std::fs::read_to_string(file_path).expect("failed to read the file"));

    loop {
        match lexer.scan_token() {
            Ok(token) => {
                if token.kind == TokenType::EndOfFile {
                    break;
                }

                println!("{token:?}");
            }
            Err(why) => println!("syntax error:\n    {why:?}\n    {why}"),
        };
    }
}
