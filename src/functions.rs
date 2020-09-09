use super::types::*;
use std::rc::Rc;

pub fn car_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj.clone(),
        Object::Cons(cons) => car_cons(&cons),
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
    }
}

fn car_cons(obj: &Cons) -> Rc<Object> {
    match &*obj {
        Cons::Some(first, _) => first.clone(),
        Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
    }
}

pub fn cdr_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj.clone(),
        Object::Cons(cons) => cdr_cons(&cons),
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
    }
}

fn cdr_cons(obj: &Cons) -> Rc<Object> {
    match &*obj {
        Cons::Some(_, second) => second.clone(),
        Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
    }
}

fn add(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value + rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
    }
}

fn sub(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value - rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
    }
}

fn mul(lhs_obj: Rc<Object>, rhs_obj: Rc<Object>) -> Rc<Object> {
    match (&*lhs_obj, &*rhs_obj) {
        (Object::Error(_), _) => lhs_obj.clone(),
        (_, Object::Error(_)) => rhs_obj.clone(),
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Rc::new(Object::Integer(Integer {
                value: lhs.value * rhs.value,
            }))
        }
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
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

fn eval_list_elements(list: &Cons, env: &Cons) -> Cons {
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
        _ => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
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
    return eval_obj(
        func.body.clone(),
        &join_two_lists_cons(
            &func.parameters,
            &eval_list_elements(args, env),
            env,
        ),
    );
}

fn eval_obj(obj: Rc<Object>, env: &Cons) -> Rc<Object> {
    match &*obj {
        Object::Error(_)
        | Object::Integer(_)
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
            message: String::from("TODO: Error message"),
        })),
    }
}

fn eval_symbol(symbol: &Symbol, env: &Cons) -> Rc<Object> {
    match env {
        Cons::Nil => Rc::new(Object::Error(Error {
            message: String::from("TODO: Error message"),
        })),
        Cons::Some(first, rest) => match &*car_obj(first.clone()) {
            Object::Symbol(found_symbol) if symbol == found_symbol => {
                cdr_obj(first.clone())
            }
            _ => match &**rest {
                Object::Cons(next_cons) => eval_symbol(symbol, &next_cons),
                _ => Rc::new(Object::Error(Error {
                    message: String::from("TODO: Error message"),
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
        Cons::Some {
            0: $first,
            1: Rc::new(Object::Cons(make_list!($($rest),*)))
        }
    };
}
