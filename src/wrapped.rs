use super::functions::*;
use super::types::*;
use std::rc::Rc;

macro_rules! simple_wrap_mayfail {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(
            args: &Cons,
            env: &Cons,
        ) -> Result<(Rc<Object>, Cons), Error> {
            match ensure_n_args(stringify!($wrapped_name), 1, args) {
                Some(err) => Err(err),
                _ => {
                    let (first_arg, env) = eval_obj(args.car(), env)?;
                    Ok(($unwrapped_name(first_arg)?, env))
                }
            }
        }
    };
}

macro_rules! simple_wrap_nofail {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(
            args: &Cons,
            env: &Cons,
        ) -> Result<(Rc<Object>, Cons), Error> {
            match ensure_n_args(stringify!($wrapped_name), 1, args) {
                Some(err) => Err(err),
                _ => {
                    let (first_arg, env) = eval_obj(args.car(), env)?;
                    Ok(($unwrapped_name(first_arg), env))
                }
            }
        }
    };
}

simple_wrap_mayfail!(wrapped_car, car_obj);
simple_wrap_mayfail!(wrapped_cdr, cdr_obj);
simple_wrap_nofail!(wrapped_not, not);
simple_wrap_mayfail!(wrapped_int_to_bool, int_to_bool);
simple_wrap_mayfail!(wrapped_bool_to_int, bool_to_int);
simple_wrap_nofail!(wrapped_is_nil, |obj: Rc<_>| Rc::new(Object::Bool(
    matches!(&*obj, Object::Cons(Cons::Nil)).into()
)));
simple_wrap_nofail!(wrapped_is_int, |obj: Rc<_>| Rc::new(Object::Bool(
    matches!(&*obj, Object::Integer(_)).into()
)));
simple_wrap_nofail!(wrapped_is_bool, |obj: Rc<_>| Rc::new(Object::Bool(
    matches!(&*obj, Object::Bool(_)).into()
)));

pub fn wrapped_quote(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match ensure_n_args("wrapped_quote", 1, args) {
        Some(err) => Err(err),
        _ => Ok((args.car(), env.clone())),
    }
}

pub fn wrapped_cons(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match ensure_n_args("wrapped_cons", 2, args) {
        Some(err) => Err(err),
        _ => {
            let (car, env) = eval_obj(args.car(), env)?;
            let (cdr, env) = eval_obj(car_obj(args.cdr())?, &env)?;
            Ok((Rc::new(Object::Cons(Cons::Some(car, cdr))), env))
        }
    }
}

pub fn wrapped_add(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match args {
        Cons::Nil => Ok((Rc::new(Object::Integer(0.into())), env.clone())),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env)?;
                let (rhs, env) = wrapped_add(rest, &env)?;
                Ok((add(lhs, rhs)?, env))
            }
            _ => Err(Error::new(String::from(
                "Arguments passed to wrapped_add must be a proper list",
            ))),
        },
    }
}

pub fn wrapped_sub(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match args.len() {
        0 => Err(Error::new(String::from(
            "wrapped_sub expected at least 1 argument but got 0",
        ))),
        1 => {
            let (rhs, env) = eval_obj(args.car(), env)?;
            Ok((sub(Rc::new(Object::Integer(0.into())), rhs)?, env))
        }
        _ => match &*args.cdr() {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(args.car(), env)?;
                let (rhs, env) = wrapped_add(rest, &env)?;
                Ok((sub(lhs, rhs)?, env))
            }
            _ => Err(Error::new(String::from(
                "Arguments passed to wrapped_sub must be a proper list",
            ))),
        },
    }
}

pub fn wrapped_mul(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match args {
        Cons::Nil => Ok((Rc::new(Object::Integer(1.into())), env.clone())),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env)?;
                let (rhs, env) = wrapped_mul(rest, &env)?;
                Ok((mul(lhs, rhs)?, env))
            }
            _ => Err(Error::new(String::from(
                "Arguments passed to wrapped_mul must be a proper list",
            ))),
        },
    }
}

pub fn wrapped_lambda(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match ensure_n_args("wrapped_lambda", 2, args) {
        Some(err) => Err(err),
        _ => match &*args.car() {
            Object::Cons(param_list) => Ok((
                Rc::new(Object::Function(Function {
                    parameters: param_list.clone(),
                    body: car_obj(args.cdr())?,
                })),
                env.clone(),
            )),
            _ => Err(Error::new(String::from(
                "First argument of lambda definition must be a list \
                            of parameters",
            ))),
        },
    }
}

pub fn wrapped_and(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match args {
        Cons::Nil => Ok((Rc::new(Object::Bool(true.into())), env.clone())),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env)?;
                if is_truthy(lhs.clone()) {
                    wrapped_and(rest, &env)
                } else {
                    Ok((lhs, env))
                }
            }
            _ => Err(Error::new(String::from(
                "Arguments passed to wrapped_and must be a proper list",
            ))),
        },
    }
}

pub fn wrapped_or(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match args {
        Cons::Nil => Ok((Rc::new(Object::Bool(false.into())), env.clone())),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env)?;
                if is_truthy(lhs.clone()) {
                    Ok((lhs, env))
                } else {
                    wrapped_or(rest, &env)
                }
            }
            _ => Err(Error::new(String::from(
                "Arguments passed to wrapped_or must be a proper list",
            ))),
        },
    }
}

pub fn wrapped_define(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match ensure_n_args("wrapped_define", 2, args) {
        Some(err) => Err(err),
        _ => match &*args.car() {
            Object::Symbol(var_name) => {
                let (var_value, env) = eval_obj(car_obj(args.cdr())?, env)?;
                Ok((
                    Rc::new(Object::Symbol(var_name.clone())),
                    Cons::Some(
                        Rc::new(Object::Cons(Cons::Some(
                            Rc::new(Object::Symbol(var_name.clone())),
                            var_value,
                        ))),
                        Rc::new(Object::Cons(env)),
                    ),
                ))
            }
            _ => Err(Error::new(String::from(
                "First argument passed to define must be a symbol",
            ))),
        },
    }
}

pub fn wrapped_if(
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match ensure_n_args("wrapped_if", 3, args) {
        Some(err) => Err(err),
        _ => {
            let (condition, env) = eval_obj(args.car(), env)?;
            if is_truthy(condition) {
                eval_obj(car_obj(args.cdr())?, &env)
            } else {
                eval_obj(car_obj(cdr_obj(args.cdr())?)?, &env)
            }
        }
    }
}
