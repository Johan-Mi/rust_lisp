use super::types::*;
use itertools::Itertools;
use std::rc::Rc;

pub fn car_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj,
        Object::Cons(cons) => match cons {
            Cons::Some(..) => cons.car(),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("car_obj", &[&*obj]))),
    }
}

pub fn cdr_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj,
        Object::Cons(cons) => match cons {
            Cons::Some(..) => cons.cdr(),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("cdr_obj", &[&*obj]))),
    }
}

pub fn add(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj,
        (_, Object::Error(_)) => rhs_obj,
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value + rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(make_type_error(
            "add",
            &[&*lhs_obj, &*rhs_obj],
        ))),
    }
}

pub fn sub(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj,
        (_, Object::Error(_)) => rhs_obj,
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value - rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(make_type_error(
            "sub",
            &[&*lhs_obj, &*rhs_obj],
        ))),
    }
}

pub fn mul(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj,
        (_, Object::Error(_)) => rhs_obj,
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value * rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(make_type_error(
            "mul",
            &[&*lhs_obj, &*rhs_obj],
        ))),
    }
}

fn join_two_lists_obj(
    first_obj: Rc<Object>,
    second_obj: Rc<Object>,
    last: &Cons,
) -> Cons {
    match (&*first_obj, &*second_obj) {
        (Object::Cons(first), Object::Cons(second)) => {
            join_two_lists_cons(first, second, last)
        }
        _ => last.clone(),
    }
}

pub fn join_two_lists_cons(first: &Cons, second: &Cons, last: &Cons) -> Cons {
    match (first, second) {
        (
            Cons::Some(first_car, first_cdr),
            Cons::Some(second_car, second_cdr),
        ) => Cons::Some {
            0: Rc::<Object>::new(Object::Cons(Cons::Some {
                0: first_car.clone(),
                1: second_car.clone(),
            })),
            1: Rc::<Object>::new(Object::Cons(join_two_lists_obj(
                first_cdr.clone(),
                second_cdr.clone(),
                last,
            ))),
        },
        _ => last.clone(),
    }
}

pub fn eval_list_elements(list: &Cons, env: &Cons) -> (Cons, Cons) {
    match list {
        Cons::Nil => (list.clone(), env.clone()),
        Cons::Some(first, second) => match &**second {
            Object::Cons(rest) => {
                let (evaluated_first, env) = eval_obj(first.clone(), env);
                let (evaluated_rest, env) = eval_list_elements(rest, &env);
                (
                    Cons::Some(
                        evaluated_first,
                        Rc::<Object>::new(Object::Cons(evaluated_rest)),
                    ),
                    env,
                )
            }
            _ => {
                let (evaluated_first, env) = eval_obj(first.clone(), env);
                (
                    Cons::Some(
                        evaluated_first,
                        Rc::new(Object::Cons(Cons::Nil)),
                    ),
                    env,
                )
            }
        },
    }
}

fn apply_obj(
    func_obj: Rc<Object>,
    args: &Cons,
    env: &Cons,
) -> (Rc<Object>, Cons) {
    match &*func_obj {
        Object::Error(_) => (func_obj, env.clone()),
        Object::Function(func) => func.apply(args, env),
        Object::BuiltinFunction(func) => func.apply(args, env),
        _ => (
            Rc::new(Object::Error(make_type_error("apply_obj", &[&*func_obj]))),
            env.clone(),
        ),
    }
}

pub fn eval_obj(obj: Rc<Object>, env: &Cons) -> (Rc<Object>, Cons) {
    match &*obj {
        Object::Error(_)
        | Object::Integer(_)
        | Object::Bool(_)
        | Object::Function(_)
        | Object::BuiltinFunction(_) => (obj, env.clone()),
        Object::Cons(cons) => match cons {
            Cons::Nil => (obj, env.clone()),
            _ => eval_cons(&cons, env),
        },
        Object::Symbol(symbol) => eval_symbol(&symbol, env),
        Object::Quote(quote) => (quote.contained.clone(), env.clone()),
    }
}

fn eval_cons(list: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
    match &*list.cdr() {
        Object::Cons(args) => {
            let (func, env) = eval_obj(list.car(), env);
            apply_obj(func, &args, &env)
        }
        _ => (
            Rc::new(Object::Error(Error {
                message: String::from(
                    "cdr of argument passed to eval_cons must be a cons",
                ),
            })),
            env.clone(),
        ),
    }
}

fn eval_symbol(symbol: &Symbol, env: &Cons) -> (Rc<Object>, Cons) {
    fn eval_symbol_internal(symbol: &Symbol, env: &Cons) -> Rc<Object> {
        match env {
            Cons::Nil => Rc::new(Object::Error(Error {
                message: format!("Unbound variable {}", symbol),
            })),
            Cons::Some(first, rest) => match &*car_obj(first.clone()) {
                Object::Symbol(found_symbol) if symbol == found_symbol => {
                    cdr_obj(first.clone())
                }
                _ => match &**rest {
                    Object::Cons(next_cons) => {
                        eval_symbol_internal(symbol, &next_cons)
                    }
                    _ => Rc::new(Object::Error(Error {
                        message: format!("Unbound variable {}", symbol),
                    })),
                },
            },
        }
    }

    (eval_symbol_internal(symbol, env), env.clone())
}

#[macro_export]
macro_rules! make_list {
    () => {
        Cons::Nil
    };

    ($first:expr $(, $rest:expr)*) => {
        Cons::Some($first, Rc::new(Object::Cons(make_list!($($rest),*))))
    };
}

fn make_type_error(func_name: &str, args: &[&Object]) -> Error {
    Error {
        message: format!(
            "{} is not callable with types ({})",
            func_name,
            args.iter()
                .copied()
                .map(Object::name_of_contained)
                .intersperse(" ")
                .collect::<String>()
        ),
    }
}

pub fn ensure_n_args(func_name: &str, n: usize, list: &Cons) -> Option<Error> {
    if !list.is_proper_list() {
        return Some(Error {
            message: format!("Call to {} must be a proper list", func_name),
        });
    }

    let length = list.len();
    if length != n {
        return Some(Error {
            message: format!(
                "{} expected {} arguments but got {}",
                func_name, n, length
            ),
        });
    }

    None
}

pub fn int_to_bool(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Integer(val) => Rc::new(Object::Bool(Bool {
            value: val.value != 0,
        })),
        _ => Rc::new(Object::Error(make_type_error("int_to_bool", &[&*obj]))),
    }
}

pub fn bool_to_int(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Bool(val) => Rc::new(Object::Integer(Integer {
            value: val.value as i32,
        })),
        _ => Rc::new(Object::Error(make_type_error("bool_to_int", &[&*obj]))),
    }
}

pub fn is_truthy(obj: Rc<Object>) -> bool {
    match &*obj {
        Object::Bool(b) => b.value,
        _ => true,
    }
}

pub fn not(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj,
        _ => Rc::new(Object::Bool(Bool {
            value: !is_truthy(obj),
        })),
    }
}
