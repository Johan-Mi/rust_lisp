use super::types::*;
use itertools::Itertools;
use std::rc::Rc;
// TODO Mutable environment

pub fn car_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj.clone(),
        Object::Cons(cons) => match cons {
            Cons::Some(..) => car_cons(&cons),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("car_obj", &[&*obj]))),
    }
}

pub fn car_cons(obj: &Cons) -> Rc<Object> {
    match &*obj {
        Cons::Some(first, _) => first.clone(),
        Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
    }
}

pub fn cdr_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj.clone(),
        Object::Cons(cons) => match cons {
            Cons::Some(..) => cdr_cons(&cons),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("cdr_obj", &[&*obj]))),
    }
}

pub fn cdr_cons(obj: &Cons) -> Rc<Object> {
    match &*obj {
        Cons::Some(_, second) => second.clone(),
        Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
    }
}

pub fn add(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
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
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
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
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
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

pub fn list_length(list: &Cons) -> usize {
    match list {
        Cons::Nil => 0,
        Cons::Some(_, next) => match &**next {
            Object::Cons(rest) => list_length(&rest) + 1,
            _ => 1,
        },
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

fn join_two_lists_cons(first: &Cons, second: &Cons, last: &Cons) -> Cons {
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

pub fn eval_list_elements(list: &Cons, env: &Cons) -> Cons {
    match list {
        Cons::Nil => list.clone(),
        Cons::Some(first, second) => match &**second {
            Object::Cons(rest) => Cons::Some {
                0: eval_obj(first.clone(), env),
                1: Rc::<Object>::new(Object::Cons(eval_list_elements(
                    rest, env,
                ))),
            },
            _ => Cons::Some {
                0: eval_obj(first.clone(), env),
                1: Rc::<Object>::new(Object::Cons(Cons::Nil)),
            },
        },
    }
}

fn apply_obj(func_obj: Rc<Object>, args: &Cons, env: &Cons) -> Rc<Object> {
    match &*func_obj {
        Object::Error(_) => func_obj,
        Object::Function(func) => apply_function(&func, args, env),
        Object::BuiltinFunction(func) => {
            apply_builtin_function(&func, args, env)
        }
        _ => {
            Rc::new(Object::Error(make_type_error("apply_obj", &[&*func_obj])))
        }
    }
}

fn apply_builtin_function(
    func: &BuiltinFunction,
    args: &Cons,
    env: &Cons,
) -> Rc<Object> {
    (func.func)(args, env)
}

fn apply_function(func: &Function, args: &Cons, env: &Cons) -> Rc<Object> {
    eval_obj(
        func.body.clone(),
        &join_two_lists_cons(
            &func.parameters,
            &eval_list_elements(args, env),
            env,
        ),
    )
}

pub fn eval_obj(obj: Rc<Object>, env: &Cons) -> Rc<Object> {
    match &*obj {
        Object::Error(_)
        | Object::Integer(_)
        | Object::Bool(_)
        | Object::Function(_)
        | Object::BuiltinFunction(_) => obj,
        Object::Cons(cons) => match cons {
            Cons::Nil => obj,
            _ => eval_cons(&cons, env),
        },
        Object::Symbol(symbol) => eval_symbol(&symbol, env),
        Object::Quote(quote) => quote.contained.clone(),
    }
}

fn eval_cons(list: &Cons, env: &Cons) -> Rc<Object> {
    match &*cdr_cons(list) {
        Object::Cons(args) => {
            apply_obj(eval_obj(car_cons(list), env), &args, env)
        }
        _ => Rc::new(Object::Error(Error {
            message: String::from(
                "car of argument passed to eval_cons must be a cons",
            ),
        })),
    }
}

fn eval_symbol(symbol: &Symbol, env: &Cons) -> Rc<Object> {
    match env {
        Cons::Nil => Rc::new(Object::Error(Error {
            message: format!("Unbound variable {}", symbol),
        })),
        Cons::Some(first, rest) => match &*car_obj(first.clone()) {
            Object::Symbol(found_symbol) if symbol == found_symbol => {
                cdr_obj(first.clone())
            }
            _ => match &**rest {
                Object::Cons(next_cons) => eval_symbol(symbol, &next_cons),
                _ => Rc::new(Object::Error(Error {
                    message: format!("Unbound variable {}", symbol),
                })),
            },
        },
    }
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

fn name_of_contained(obj: &Object) -> &str {
    match obj {
        Object::Integer(_) => "(type int)",
        Object::Symbol(_) => "(type symbol)",
        Object::Error(_) => "(type error)",
        Object::Function(_) => "(type function)",
        Object::BuiltinFunction(_) => "(type builtin-function)",
        Object::Quote(_) => "(type quote)",
        Object::Cons(_) => "(type cons)",
        Object::Bool(_) => "(type bool)",
    }
}

fn make_type_error(func_name: &str, args: &[&Object]) -> Error {
    Error {
        message: format!(
            "{} is not callable with types ({})",
            func_name,
            args.iter()
                .copied()
                .map(name_of_contained)
                .intersperse(" ")
                .collect::<String>()
        ),
    }
}

fn is_proper_list(list: &Cons) -> bool {
    match list {
        Cons::Nil => true,
        Cons::Some(_, next) => match &**next {
            Object::Cons(rest) => is_proper_list(&rest),
            _ => false,
        },
    }
}

pub fn ensure_n_args(func_name: &str, n: usize, list: &Cons) -> Option<Error> {
    if !is_proper_list(list) {
        return Some(Error {
            message: format!("Call to {} must be a proper list", func_name),
        });
    }

    let length = list_length(list);
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
    Rc::new(Object::Bool(Bool {
        value: !is_truthy(obj),
    }))
}
