use super::error::*;
use super::object::*;
use std::fmt;
use std::rc::Rc;

/// The `Cons` cell either contains pointers to two `Object`s, or nothing
/// (`Nil`). It's one of the simplest persistent data structures, but very
/// versatile. Multiple `Cons`es can for example be connected into a linked list
/// or a tree.
#[derive(Clone)]
pub enum Cons {
    Some(Rc<Object>, Rc<Object>),
    Nil,
}

impl Cons {
    /// Returns the length of the linked list that `self` is the beginning of.
    pub fn len(&self) -> usize {
        match self {
            Cons::Nil => 0,
            Cons::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.len() + 1,
                _ => 1,
            },
        }
    }

    /// Returns the first element of `self`, or `Nil` if `self` is `Nil`.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = Rc::new(Object::Integer(Integer { value: 1 }));
    /// let two = Rc::new(Object::Integer(Integer { value: 2 }));
    ///
    /// let a = Cons::Some(one, two);
    ///
    /// assert_eq!(a.car(), one);
    /// ```
    ///
    /// ```
    /// let nil = Rc::new(Object::Cons(Cons::Nil));
    ///
    /// let a = Cons::Nil;
    ///
    /// assert_eq!(a.car(), nil);
    /// ```
    pub fn car(&self) -> Rc<Object> {
        match self {
            Cons::Some(first, _) => first.clone(),
            Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
        }
    }

    /// Returns the second element of `self`, or `Nil` if `self` is `Nil`.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = Rc::new(Object::Integer(Integer { value: 1 }));
    /// let two = Rc::new(Object::Integer(Integer { value: 2 }));
    ///
    /// let a = Cons::Some(one, two);
    ///
    /// assert_eq!(a.cdr(), two);
    /// ```
    ///
    /// ```
    /// let nil = Rc::new(Object::Cons(Cons::Nil));
    ///
    /// let a = Cons::Nil;
    ///
    /// assert_eq!(a.cdr(), nil);
    /// ```
    pub fn cdr(&self) -> Rc<Object> {
        match self {
            Cons::Some(_, second) => second.clone(),
            Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
        }
    }

    /// Checks if `self` is a linked list. `self` is a linked list if it's `Nil`
    /// or if its `cdr` is a linked list.
    pub fn is_proper_list(&self) -> bool {
        match self {
            Cons::Nil => true,
            Cons::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.is_proper_list(),
                _ => false,
            },
        }
    }

    /// Evaluates `self` with variables from `env`. This returns the result of
    /// calling `self.car()` with arguments from `self.cdr()`
    pub fn eval(&self, env: &Cons) -> (Rc<Object>, Cons) {
        match &*self.cdr() {
            Object::Cons(args) => {
                let (func, env) = eval_obj(self.car(), env);
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
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fn to_cons_string(obj: &Object) -> String {
            match obj {
                Object::Cons(cons) => match cons {
                    Cons::Nil => String::new(),
                    Cons::Some(first, second) => {
                        format!(" {}{}", first, to_cons_string(second))
                    }
                },
                _ => format!(" . {}", obj),
            }
        }

        match self {
            Cons::Nil => write!(formatter, "()"),
            Cons::Some(first, second) => {
                write!(formatter, "({}{})", first, to_cons_string(second))
            }
        }
    }
}
