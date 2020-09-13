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

macro_rules! make_env {
    ($($name:literal = $value:expr),*) => {
        make_list![
        $(
            Rc::new(Object::Cons(Cons::Some(
                Rc::new(Object::Symbol(Symbol {
                    name: String::from($name),
                })),
                $value
            )))
        ),*
        ]
    }
}

macro_rules! make_builtin_function {
    ($func:expr) => {
        Rc::new(Object::BuiltinFunction(BuiltinFunction { func: $func }))
    };
}

fn main() {
    let env = make_env![
        "car" = make_builtin_function!(wrapped_car),
        "cdr" = make_builtin_function!(wrapped_cdr),
        "cons" = make_builtin_function!(wrapped_cons),
        "lambda" = make_builtin_function!(wrapped_lambda),
        "+" = make_builtin_function!(wrapped_add),
        "-" = make_builtin_function!(wrapped_sub),
        "*" = make_builtin_function!(wrapped_mul),
        "quote" = make_builtin_function!(wrapped_quote),
        "int->bool" = make_builtin_function!(wrapped_int_to_bool),
        "bool->int" = make_builtin_function!(wrapped_bool_to_int),
        "and" = make_builtin_function!(wrapped_and),
        "or" = make_builtin_function!(wrapped_or),
        "not" = make_builtin_function!(wrapped_not),
        "true" = Rc::new(Object::Bool(Bool { value: true })),
        "false" = Rc::new(Object::Bool(Bool { value: false }))
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
