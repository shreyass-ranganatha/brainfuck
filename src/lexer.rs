
#[derive(Clone)]
#[derive(Debug)]
pub enum TokenType {
    RightShift,
    LeftShift,
    Plus,
    Minus,
    Read,
    Print,
    LeftBracket,
    RightBracket,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub type_: TokenType
}

pub fn tokenize(stream: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i = 0;

    while i < stream.len() {
        let slice = &stream[i..i+1];

        match slice {
            ">" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::RightShift}),
            "<" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::LeftShift}),
            "+" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::Plus}),
            "-" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::Minus}),
            "," => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::Read}),
            "." => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::Print}),
            "[" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::LeftBracket}),
            "]" => tokens.push(Token {lexeme: slice.to_string(), type_: TokenType::RightBracket}),
            _ => {}
        }

        i += 1;
    }

    tokens
}
