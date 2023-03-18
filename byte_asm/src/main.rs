use byte_asm::lex::Lexer;

fn main() {
    if let Some(file_path) = std::env::args().nth(1) {
        let mut lexer =
            Lexer::new(std::fs::read_to_string(file_path).expect("failed to read the file"));

        match lexer.scan_tokens() {
            Ok(data) => println!("{data:#?}"),
            Err(why) => println!("{why:?}"),
        }
    }
}
