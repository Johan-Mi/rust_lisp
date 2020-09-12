use super::functions::*;
use super::types::*;
use std::rc::Rc;

pub fn wrapped_car(args: &Cons, env: &Cons) -> Rc<Object> {
    match ensure_n_args("wrapped_car", 1, args) {
        Some(err) => Rc::new(Object::Error(err)),
        _ => car_obj(eval_obj(car_cons(args), env)),
    }
}

pub fn wrapped_cdr(args: &Cons, env: &Cons) -> Rc<Object> {
    match ensure_n_args("wrapped_cdr", 1, args) {
        Some(err) => Rc::new(Object::Error(err)),
        _ => cdr_obj(eval_obj(car_cons(args), env)),
    }
}

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
