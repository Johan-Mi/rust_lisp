use super::functions::*;
use super::types::*;
use std::rc::Rc;

macro_rules! simple_wrap_1 {
    ($wrapped_name:ident, $unwrapped_name:expr) => {
        pub fn $wrapped_name(args: &Cons, env: &Cons) -> Rc<Object> {
            match ensure_n_args(stringify!($wrapped_name), 1, args) {
                Some(err) => Rc::new(Object::Error(err)),
                _ => $unwrapped_name(eval_obj(car_cons(args), env)),
            }
        }
    };
}

simple_wrap_1!(wrapped_car, car_obj);
simple_wrap_1!(wrapped_cdr, cdr_obj);
simple_wrap_1!(wrapped_not, not);
simple_wrap_1!(wrapped_int_to_bool, int_to_bool);
simple_wrap_1!(wrapped_bool_to_int, bool_to_int);

pub fn wrapped_quote(args: &Cons, _env: &Cons) -> Rc<Object> {
    match ensure_n_args("wrapped_quote", 1, args) {
        Some(err) => Rc::new(Object::Error(err)),
        _ => car_cons(args),
    }
}

pub fn wrapped_cons(args: &Cons, env: &Cons) -> Rc<Object> {
    match ensure_n_args("wrapped_cons", 2, args) {
        Some(err) => Rc::new(Object::Error(err)),
        _ => Rc::new(Object::Cons(Cons::Some(
            eval_obj(car_cons(args), env),
            eval_obj(car_obj(cdr_cons(args)), env),
        ))),
    }
}

pub fn wrapped_add(args: &Cons, env: &Cons) -> Rc<Object> {
    match args {
        Cons::Nil => Rc::new(Object::Integer(Integer { value: 0 })),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                add(eval_obj(car.clone(), env), wrapped_add(rest, env))
            }
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "Arguments passed to wrapped_add must be a proper list",
                ),
            })),
        },
    }
}

pub fn wrapped_sub(args: &Cons, env: &Cons) -> Rc<Object> {
    match list_length(args) {
        0 => Rc::new(Object::Error(Error {
            message: String::from(
                "wrapped_sub expected at least 1 argument but got 0",
            ),
        })),
        1 => sub(
            Rc::new(Object::Integer(Integer { value: 0 })),
            eval_obj(car_cons(args), env),
        ),
        _ => match &*cdr_cons(args) {
            Object::Cons(rest) => {
                sub(eval_obj(car_cons(args), env), wrapped_add(&rest, env))
            }
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "Arguments passed to wrapped_sub must be a proper list",
                ),
            })),
        },
    }
}

pub fn wrapped_mul(args: &Cons, env: &Cons) -> Rc<Object> {
    match args {
        Cons::Nil => Rc::new(Object::Integer(Integer { value: 1 })),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                mul(eval_obj(car.clone(), env), wrapped_mul(rest, env))
            }
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "Arguments passed to wrapped_mul must be a proper list",
                ),
            })),
        },
    }
}

pub fn wrapped_lambda(args: &Cons, _env: &Cons) -> Rc<Object> {
    match ensure_n_args("wrapped_lambda", 2, args) {
        Some(err) => Rc::new(Object::Error(err)),
        _ => match &*car_cons(args) {
            Object::Error(_) => car_cons(args),
            Object::Cons(param_list) => Rc::new(Object::Function(Function {
                parameters: param_list.clone(),
                body: car_obj(cdr_cons(args)),
            })),
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "First argument of lambda definition must be a list \
                            of parameters",
                ),
            })),
        },
    }
}

pub fn wrapped_and(args: &Cons, env: &Cons) -> Rc<Object> {
    match args {
        Cons::Nil => Rc::new(Object::Bool(Bool { value: true })),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let lhs = eval_obj(car.clone(), env);
                if is_truthy(lhs.clone()) {
                    wrapped_and(rest, env)
                } else {
                    lhs
                }
            }
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "Arguments passed to wrapped_and must be a proper list",
                ),
            })),
        },
    }
}

pub fn wrapped_or(args: &Cons, env: &Cons) -> Rc<Object> {
    match args {
        Cons::Nil => Rc::new(Object::Bool(Bool { value: false })),
        Cons::Some(car, cdr) => match &**cdr {
            Object::Cons(rest) => {
                let lhs = eval_obj(car.clone(), env);
                if is_truthy(lhs.clone()) {
                    lhs
                } else {
                    wrapped_or(rest, env)
                }
            }
            _ => Rc::new(Object::Error(Error {
                message: String::from(
                    "Arguments passed to wrapped_or must be a proper list",
                ),
            })),
        },
    }
}
