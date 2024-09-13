#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

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
        "car" = builtin_function(wrapped::car),
        "cdr" = builtin_function(wrapped::cdr),
        "cons" = builtin_function(wrapped::cons),
        "lambda" = builtin_function(wrapped::lambda),
        "+" = builtin_function(wrapped::add),
        "-" = builtin_function(wrapped::sub),
        "*" = builtin_function(wrapped::mul),
        "quote" = builtin_function(wrapped::quote),
        "int->bool" = builtin_function(wrapped::int_to_bool),
        "bool->int" = builtin_function(wrapped::bool_to_int),
        "and" = builtin_function(wrapped::and),
        "or" = builtin_function(wrapped::or),
        "not" = builtin_function(wrapped::not),
        "define" = builtin_function(wrapped::define),
        "nil?" = builtin_function(wrapped::is_nil),
        "int?" = builtin_function(wrapped::is_int),
        "bool?" = builtin_function(wrapped::is_bool),
        "if" = builtin_function(wrapped::r#if),
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
