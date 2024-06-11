mod lexer;
mod parser;
mod runtime;
mod interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(script) = args.get(1) {
        let mut rt = runtime::Runtime::default();
    
        let tokens = lexer::tokenize(script.clone());
        let expr = &parser::parse(tokens);
    
        // dbg!(expr);
        interpreter::interpret(&expr.0, &mut rt);

    } else {
        println!("Enter your script as a positional argument");
    }

}

