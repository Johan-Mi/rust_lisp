use std::rc::Rc;
use to_string::*;
use types::*;
mod functions;
mod lexer;
mod to_string;
mod types;

fn main() {
    let a = Rc::new(Object::Cons(make_list![
        Rc::new(Object::Integer(Integer { value: 1 })),
        Rc::new(Object::Integer(Integer { value: 2 })),
        Rc::new(Object::Integer(Integer { value: 3 })),
        Rc::new(Object::Integer(Integer { value: 4 })),
        Rc::new(Object::Integer(Integer { value: 5 }))
    ]));

    println!("{}", to_string_obj(a.clone()));
}
