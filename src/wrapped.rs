use crate::{functions, types::*};
use anyhow::{bail, Result};
use std::rc::Rc;

macro_rules! wrap_fallible {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(
            args: &Cons,
            env: &Cons,
        ) -> Result<(Rc<Object>, Cons)> {
            functions::ensure_n_args(stringify!($wrapped_name), 1, args)?;
            let (first_arg, env) = args.car().eval(env)?;
            Ok(($unwrapped_name(first_arg)?, env))
        }
    };
}

macro_rules! wrap_infallible {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(
            args: &Cons,
            env: &Cons,
        ) -> Result<(Rc<Object>, Cons)> {
            functions::ensure_n_args(stringify!($wrapped_name), 1, args)?;
            let (first_arg, env) = args.car().eval(env)?;
            Ok(($unwrapped_name(first_arg), env))
        }
    };
}

wrap_fallible!(car, Object::car);
wrap_fallible!(cdr, Object::cdr);
wrap_infallible!(not, |obj: Rc<_>| functions::not(&obj));
wrap_fallible!(int_to_bool, |obj: Rc<_>| functions::int_to_bool(&obj));
wrap_fallible!(bool_to_int, |obj: Rc<_>| functions::bool_to_int(&obj));
wrap_infallible!(is_nil, |obj: Rc<_>| Rc::new(Object::Bool(matches!(
    &*obj,
    Object::Cons(Cons(None))
))));
wrap_infallible!(is_int, |obj: Rc<_>| Rc::new(Object::Bool(matches!(
    &*obj,
    Object::Integer(_)
))));
wrap_infallible!(is_bool, |obj: Rc<_>| Rc::new(Object::Bool(matches!(
    &*obj,
    Object::Bool(_)
))));

pub fn quote(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    functions::ensure_n_args("wrapped_quote", 1, args)?;
    Ok((args.car(), env.clone()))
}

pub fn cons(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    functions::ensure_n_args("wrapped_cons", 2, args)?;
    let (car, env) = args.car().eval(env)?;
    let (cdr, env) = args.cdr().car()?.eval(&env)?;
    Ok((Rc::new(Object::Cons(Cons(Some((car, cdr))))), env))
}

pub fn add(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    match &args.0 {
        None => Ok((Rc::new(Object::Integer(0)), env.clone())),
        Some((car, cdr)) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = car.clone().eval(env)?;
                let (rhs, env) = add(rest, &env)?;
                Ok((Rc::new(functions::add(&lhs, &rhs)?), env))
            }
            _ => bail!("arguments passed to wrapped_add must be a proper list"),
        },
    }
}

pub fn sub(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    match args.len() {
        0 => bail!("wrapped_sub expected at least 1 argument but got 0"),
        1 => {
            let (rhs, env) = args.car().eval(env)?;
            Ok((Rc::new(functions::sub(&Object::Integer(0), &rhs)?), env))
        }
        _ => match &*args.cdr() {
            Object::Cons(rest) => {
                let (lhs, env) = args.car().eval(env)?;
                let (rhs, env) = add(rest, &env)?;
                Ok((Rc::new(functions::sub(&lhs, &rhs)?), env))
            }
            _ => bail!("arguments passed to wrapped_sub must be a proper list"),
        },
    }
}

pub fn mul(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    match &args.0 {
        None => Ok((Rc::new(Object::Integer(1)), env.clone())),
        Some((car, cdr)) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = car.clone().eval(env)?;
                let (rhs, env) = mul(rest, &env)?;
                Ok((Rc::new(functions::mul(&lhs, &rhs)?), env))
            }
            _ => bail!("arguments passed to wrapped_mul must be a proper list"),
        },
    }
}

pub fn lambda(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    functions::ensure_n_args("wrapped_lambda", 2, args)?;
    match &*args.car() {
        Object::Cons(param_list) => Ok((
            Rc::new(Object::Function(Function::new(
                param_list.clone(),
                args.cdr().car()?,
            ))),
            env.clone(),
        )),
        _ => bail!(
            "first argument of lambda definition must be a list of parameters"
        ),
    }
}

pub fn and(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    match &args.0 {
        None => Ok((Rc::new(Object::Bool(true)), env.clone())),
        Some((car, cdr)) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = car.clone().eval(env)?;
                if functions::is_truthy(&lhs) {
                    and(rest, &env)
                } else {
                    Ok((lhs, env))
                }
            }
            _ => bail!("arguments passed to wrapped_and must be a proper list"),
        },
    }
}

pub fn or(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    match &args.0 {
        None => Ok((Rc::new(Object::Bool(false)), env.clone())),
        Some((car, cdr)) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = car.clone().eval(env)?;
                if functions::is_truthy(&lhs) {
                    Ok((lhs, env))
                } else {
                    or(rest, &env)
                }
            }
            _ => bail!("arguments passed to wrapped_or must be a proper list"),
        },
    }
}

pub fn define(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    functions::ensure_n_args("wrapped_define", 2, args)?;
    match &*args.car() {
        Object::Symbol(var_name) => {
            let (var_value, env) = args.cdr().car()?.eval(env)?;
            Ok((
                Rc::new(Object::Symbol(var_name.clone())),
                Cons(Some((
                    Rc::new(Object::Cons(Cons(Some((
                        Rc::new(Object::Symbol(var_name.clone())),
                        var_value,
                    ))))),
                    Rc::new(Object::Cons(env)),
                ))),
            ))
        }
        _ => bail!("first argument passed to define must be a symbol"),
    }
}

pub fn r#if(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
    functions::ensure_n_args("wrapped_if", 3, args)?;
    let (condition, env) = args.car().eval(env)?;
    if functions::is_truthy(&condition) {
        args.cdr().car()?.eval(&env)
    } else {
        args.cdr().cdr()?.car()?.eval(&env)
    }
}
