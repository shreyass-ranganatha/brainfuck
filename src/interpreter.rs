use crate::lexer::TokenType;
use crate::runtime::Runtime;
use crate::parser::Expr;

pub fn interpret(expr: &Expr, rt: &mut Runtime) {
    let mut ex = expr.next();

    while let Some(e) = ex {
        match &**e {
            Expr::Expr(Some(a), _) => {
                match a.type_ {
                    TokenType::RightShift => rt.shift_right(),
                    TokenType::LeftShift => rt.shift_left(),
                    TokenType::Plus => rt.plus(),
                    TokenType::Minus => rt.minus(),
                    TokenType::Read => rt.read(),
                    TokenType::Print => rt.print(),
                    _ => ()
                }
            },
            
            Expr::Loop(a, _) => {
                loop {
                    interpret(&a, rt);

                    if rt.value() == 0 {
                        break;
                    }
                }
            },
            
            _ => ()
        }

        ex = ex.as_ref().unwrap().next();
    }
    
}
