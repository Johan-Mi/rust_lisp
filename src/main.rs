#![forbid(unsafe_code)]
#![warn(clippy::nursery)]

mod functions;
mod lexer;
mod parser;
mod types;
mod wrapped;
use anyhow::{Context, Result};
use lexer::lex;
use parser::parse_expressions;
use std::{fs, rc::Rc};
use types::{BuiltinFunction, Cons, Object};
use wrapped::*;

macro_rules! make_env {
    ($($name:literal = $value:expr),*) => {
        make_list![
        $(
            Rc::new(Object::Cons(Cons::Some(
                Rc::new(Object::Symbol($name.parse().unwrap())),
                $value
            )))
        ),*
        ]
    }
}

fn main() -> Result<()> {
    let builtin_function =
        |func| Rc::new(Object::BuiltinFunction(BuiltinFunction(func)));

    let mut env = make_env![
        "car" = builtin_function(wrapped_car),
        "cdr" = builtin_function(wrapped_cdr),
        "cons" = builtin_function(wrapped_cons),
        "lambda" = builtin_function(wrapped_lambda),
        "+" = builtin_function(wrapped_add),
        "-" = builtin_function(wrapped_sub),
        "*" = builtin_function(wrapped_mul),
        "quote" = builtin_function(wrapped_quote),
        "int->bool" = builtin_function(wrapped_int_to_bool),
        "bool->int" = builtin_function(wrapped_bool_to_int),
        "and" = builtin_function(wrapped_and),
        "or" = builtin_function(wrapped_or),
        "not" = builtin_function(wrapped_not),
        "define" = builtin_function(wrapped_define),
        "nil?" = builtin_function(wrapped_is_nil),
        "int?" = builtin_function(wrapped_is_int),
        "bool?" = builtin_function(wrapped_is_bool),
        "if" = builtin_function(wrapped_if),
        "true" = Rc::new(Object::Bool(true)),
        "false" = Rc::new(Object::Bool(false))
    ];

    let source_code = fs::read_to_string("program.lisp")
        .context("failed to read source file")?;

    let lexed = lex(&source_code);
    let (exprs, _) =
        parse_expressions(&lexed).context("failed to parse source code")?;

    for e in exprs {
        println!("{e}");
        match Rc::new(e).eval(&env) {
            Ok((result, new_env)) => {
                env = new_env;
                println!("=> {result}");
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    Ok(())
}
