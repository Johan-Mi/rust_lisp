use super::functions::*;
use super::types::*;
use std::rc::Rc;

macro_rules! simple_wrap_1 {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
            match ensure_n_args(stringify!($wrapped_name), 1, args) {
                Some(err) => (Rc::new(Object::Error(err)), env.clone()),
                _ => {
                    let (first_arg, env) = eval_obj(args.car(), env);
                    ($unwrapped_name(first_arg), env)
                }
            }
        }
    };
}

simple_wrap_1!(wrapped_car, car_obj);
simple_wrap_1!(wrapped_cdr, cdr_obj);
simple_wrap_1!(wrapped_not, not);
simple_wrap_1!(wrapped_int_to_bool, int_to_bool);
simple_wrap_1!(wrapped_bool_to_int, bool_to_int);

pub fn wrapped_quote(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match ensure_n_args("wrapped_quote", 1, args) {
        Some(err) => (Rc::new(Object::Error(err)), env.clone()),
        _ => (args.car(), env.clone()),
    }
}

pub fn wrapped_cons(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match ensure_n_args("wrapped_cons", 2, args) {
        Some(err) => (Rc::new(Object::Error(err)), env.clone()),
        _ => {
            let (car, env) = eval_obj(args.car(), env);
            let (cdr, env) = eval_obj(car_obj(args.cdr()), &env);
            match (&*car, &*cdr) {
                (Object::Error(_), _) => (car, env),
                (_, Object::Error(_)) => (cdr, env),
                _ => (Rc::new(Object::Cons(Cons::Some(car, cdr))), env),
            }
        }
    }
}

pub fn wrapped_add(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match args {
        Cons::Nil => {
            (Rc::new(Object::Integer(Integer { value: 0 })), env.clone())
        }
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env);
                let (rhs, env) = wrapped_add(rest, &env);
                (add(lhs, rhs), env)
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "Arguments passed to wrapped_add must be a proper list",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_sub(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match args.len() {
        0 => (
            Rc::new(Object::Error(Error {
                message: String::from(
                    "wrapped_sub expected at least 1 argument but got 0",
                ),
            })),
            env.clone(),
        ),
        1 => {
            let (rhs, env) = eval_obj(args.car(), env);
            (
                sub(Rc::new(Object::Integer(Integer { value: 0 })), rhs),
                env,
            )
        }
        _ => match &*args.cdr() {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(args.car(), env);
                let (rhs, env) = wrapped_add(rest, &env);
                (sub(lhs, rhs), env)
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "Arguments passed to wrapped_sub must be a proper list",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_mul(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match args {
        Cons::Nil => {
            (Rc::new(Object::Integer(Integer { value: 1 })), env.clone())
        }
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env);
                let (rhs, env) = wrapped_mul(rest, &env);
                (mul(lhs, rhs), env)
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "Arguments passed to wrapped_mul must be a proper list",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_lambda(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match ensure_n_args("wrapped_lambda", 2, args) {
        Some(err) => (Rc::new(Object::Error(err)), env.clone()),
        _ => match &*args.car() {
            Object::Error(_) => (args.car(), env.clone()),
            Object::Cons(param_list) => (
                Rc::new(Object::Function(Function {
                    parameters: param_list.clone(),
                    body: car_obj(args.cdr()),
                })),
                env.clone(),
            ),
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "First argument of lambda definition must be a list \
                            of parameters",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_and(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match args {
        Cons::Nil => (Rc::new(Object::Bool(Bool { value: true })), env.clone()),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env);
                match &*lhs {
                    Object::Error(_) => (lhs, env),
                    _ => {
                        if is_truthy(lhs.clone()) {
                            wrapped_and(rest, &env)
                        } else {
                            (lhs, env)
                        }
                    }
                }
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "Arguments passed to wrapped_and must be a proper list",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_or(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match args {
        Cons::Nil => {
            (Rc::new(Object::Bool(Bool { value: false })), env.clone())
        }
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let (lhs, env) = eval_obj(car.clone(), env);
                match &*lhs {
                    Object::Error(_) => (lhs, env),
                    _ => {
                        if is_truthy(lhs.clone()) {
                            (lhs, env)
                        } else {
                            wrapped_or(rest, &env)
                        }
                    }
                }
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "Arguments passed to wrapped_or must be a proper list",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}

pub fn wrapped_define(args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match ensure_n_args("wrapped_define", 2, args) {
        Some(err) => (Rc::new(Object::Error(err)), env.clone()),
        _ => match &*args.car() {
            Object::Error(_) => (args.car(), env.clone()),
            Object::Symbol(var_name) => {
                let (var_value, env) = eval_obj(car_obj(args.cdr()), env);
                match &*var_value {
                    Object::Error(_) => (var_value, env),
                    _ => (
                        Rc::new(Object::Symbol(var_name.clone())),
                        Cons::Some(
                            Rc::new(Object::Cons(Cons::Some(
                                Rc::new(Object::Symbol(var_name.clone())),
                                var_value,
                            ))),
                            Rc::new(Object::Cons(env)),
                        ),
                    ),
                }
            }
            _ => (
                Rc::new(Object::Error(Error {
                    message: String::from(
                        "First argument passed to define must be a symbol",
                    ),
                })),
                env.clone(),
            ),
        },
    }
}
