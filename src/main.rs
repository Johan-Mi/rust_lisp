mod functions;
mod lexer;
mod parser;
mod types;
mod wrapped;
use lexer::*;
use parser::*;
use std::fs;
use std::rc::Rc;
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
        Rc::new(Object::BuiltinFunction(BuiltinFunction::new($func)))
    };
}

fn main() {
    let mut env = make_env![
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
        "define" = make_builtin_function!(wrapped_define),
        "nil?" = make_builtin_function!(wrapped_is_nil),
        "int?" = make_builtin_function!(wrapped_is_int),
        "bool?" = make_builtin_function!(wrapped_is_bool),
        "if" = make_builtin_function!(wrapped_if),
        "true" = Rc::new(Object::Bool(true.into())),
        "false" = Rc::new(Object::Bool(false.into()))
    ];

    match fs::read_to_string("program.lisp") {
        Ok(program_str) => {
            let lexed = lex(&program_str);
            match parse_expressions(&lexed) {
                Some((exprs, _)) => {
                    for e in exprs {
                        let (result, new_env) = eval_obj(e.clone(), &env);
                        env = new_env;
                        println!("{}", e);
                        println!("=> {}", result);
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
