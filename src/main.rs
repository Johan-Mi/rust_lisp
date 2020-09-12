mod functions;
mod lexer;
mod parser;
mod to_string;
mod types;
mod wrapped;
use functions::*;
use lexer::*;
use parser::*;
use std::fs;
use std::rc::Rc;
use to_string::*;
use types::*;
use wrapped::*;

fn main() {
    let env = make_list![
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("car")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_car
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("cdr")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_cdr
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("cons")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_cons
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("lambda")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_lambda
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("+")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_add
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("-")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_sub
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("*")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_mul
            }))
        ))),
        Rc::new(Object::Cons(Cons::Some(
            Rc::new(Object::Symbol(Symbol {
                name: String::from("quote")
            })),
            Rc::new(Object::BuiltinFunction(BuiltinFunction {
                func: wrapped_quote
            }))
        )))
    ];

    match fs::read_to_string("program.lisp") {
        Ok(program_str) => {
            let lexed = lex(&program_str);
            match parse_expressions(&lexed) {
                Some((exprs, _)) => {
                    for e in exprs {
                        println!("{}", to_string_obj(e.clone()));
                        println!(
                            "=> {}",
                            to_string_obj(eval_obj(e.clone(), &env))
                        );
                    }
                }
                _ => {
                    eprintln!("Couldn't parse file");
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
