use crate::types::*;
use anyhow::{anyhow, ensure, Error, Result};
use itertools::Itertools;
use std::rc::Rc;

pub fn add(lhs_obj: &Object, rhs_obj: &Object) -> Result<Object> {
    match (lhs_obj, rhs_obj) {
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Ok(Object::Integer(lhs + rhs))
        }
        _ => Err(make_type_error("add", &[lhs_obj, rhs_obj])),
    }
}

pub fn sub(lhs_obj: &Object, rhs_obj: &Object) -> Result<Object> {
    match (lhs_obj, rhs_obj) {
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Ok(Object::Integer(lhs - rhs))
        }
        _ => Err(make_type_error("sub", &[lhs_obj, rhs_obj])),
    }
}

pub fn mul(lhs_obj: &Object, rhs_obj: &Object) -> Result<Object> {
    match (lhs_obj, rhs_obj) {
        (Object::Integer(lhs), Object::Integer(rhs)) => {
            Ok(Object::Integer(lhs * rhs))
        }
        _ => Err(make_type_error("mul", &[lhs_obj, rhs_obj])),
    }
}

fn join_two_lists_obj(
    first_obj: &Object,
    second_obj: &Object,
    last: &Cons,
) -> Cons {
    match (first_obj, second_obj) {
        (Object::Cons(first), Object::Cons(second)) => {
            join_two_lists_cons(first, second, last)
        }
        _ => last.clone(),
    }
}

pub fn join_two_lists_cons(first: &Cons, second: &Cons, last: &Cons) -> Cons {
    match (&first.0, &second.0) {
        (Some((first_car, first_cdr)), Some((second_car, second_cdr))) => {
            Cons(Some((
                Rc::new(Object::Cons(Cons(Some((
                    first_car.clone(),
                    second_car.clone(),
                ))))),
                Rc::new(Object::Cons(join_two_lists_obj(
                    first_cdr, second_cdr, last,
                ))),
            )))
        }
        _ => last.clone(),
    }
}

pub fn eval_list_elements(list: &Cons, env: &Cons) -> Result<(Cons, Cons)> {
    match &list.0 {
        None => Ok((list.clone(), env.clone())),
        Some((first, second)) => {
            let (evaluated_first, env) = first.clone().eval(env)?;
            if let Object::Cons(rest) = &**second {
                let (evaluated_rest, env) = eval_list_elements(rest, &env)?;
                Ok((
                    Cons(Some((
                        evaluated_first,
                        Rc::<Object>::new(Object::Cons(evaluated_rest)),
                    ))),
                    env,
                ))
            } else {
                Ok((
                    Cons(Some((
                        evaluated_first,
                        Rc::new(Object::Cons(Cons(None))),
                    ))),
                    env,
                ))
            }
        }
    }
}

#[macro_export]
macro_rules! make_list {
    () => {
        Cons(None)
    };

    ($first:expr $(, $rest:expr)*) => {
        Cons(Some(($first, Rc::new(Object::Cons(make_list!($($rest),*))))))
    };
}

pub fn make_type_error(func_name: &str, args: &[&Object]) -> Error {
    anyhow!(
        "{func_name} is not callable with types ({})",
        args.iter()
            .copied()
            .map(Object::name_of_contained)
            .join(" ")
    )
}

pub fn ensure_n_args(func_name: &str, n: usize, list: &Cons) -> Result<()> {
    ensure!(
        list.is_proper_list(),
        "call to {func_name} must be a proper list",
    );

    let length = list.len();
    ensure!(
        length == n,
        "{func_name} expected {n} arguments but got {length}"
    );

    Ok(())
}

pub fn int_to_bool(obj: &Object) -> Result<Rc<Object>> {
    match obj {
        Object::Integer(val) => Ok(Rc::new(Object::Bool(*val != 0))),
        _ => Err(make_type_error("int_to_bool", &[obj])),
    }
}

pub fn bool_to_int(obj: &Object) -> Result<Rc<Object>> {
    match obj {
        Object::Bool(val) => Ok(Rc::new(Object::Integer((*val).into()))),
        _ => Err(make_type_error("bool_to_int", &[obj])),
    }
}

pub const fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Bool(b) => *b,
        _ => true,
    }
}

pub fn not(obj: &Object) -> Rc<Object> {
    Rc::new(Object::Bool(!is_truthy(obj)))
}
