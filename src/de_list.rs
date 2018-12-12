use crate::de_list::DeList::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum DeList<T> {
    Cons(RefCell<Rc<DeList<T>>>, T, RefCell<Rc<DeList<T>>>),
    Nil,
}

impl<T> DeList<T> {
    pub fn new(value: T) -> Self {
        Cons(
            RefCell::new(Rc::new(Nil)),
            value,
            RefCell::new(Rc::new(Nil)),
        )
    }

    pub fn value(&self) -> &T {
        match self {
            Cons(_, v, _) => v,
            Nil => panic!("nil"),
        }
    }

    pub fn left(&self) -> Rc<Self> {
        match self {
            Cons(left, _, _) => left.borrow().clone(),
            Nil => panic!("nil"),
        }
    }

    pub fn right(&self) -> Rc<Self> {
        match self {
            Cons(_, _, right) => right.borrow().clone(),
            Nil => panic!("nil"),
        }
    }

    pub fn set_left(&self, other: &Rc<Self>) {
        match self {
            Cons(left, _, _) => *left.borrow_mut() = Rc::clone(other),
            Nil => panic!("nil"),
        }
    }

    pub fn set_right(&self, other: &Rc<Self>) {
        match self {
            Cons(_, _, right) => *right.borrow_mut() = Rc::clone(other),
            Nil => panic!("nil"),
        }
    }
}
