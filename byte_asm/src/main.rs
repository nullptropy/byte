use byte_asm::lex::Lexer;

fn main() {
    let path = std::env::args().nth(1).unwrap_or("test.s".to_string());
    let source = std::fs::read_to_string(path).expect("failed to read the file");
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();

    loop {
        match lexer.scan_token() {
            Ok(token) if token.eof() => break,
            Ok(token) => tokens.push(token),
            Err(why) => println!("syntax error:\n    {why:?}\n    {why}"),
        };
    }

    println!("{tokens:#?}");
}
