use crate::ring::DeList::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum DeList<T> {
    Cons(RefCell<Rc<DeList<T>>>, T, RefCell<Rc<DeList<T>>>),
    Nil,
}

impl<T> DeList<T> {
    fn new(value: T) -> Self {
        Cons(
            RefCell::new(Rc::new(Nil)),
            value,
            RefCell::new(Rc::new(Nil)),
        )
    }

    fn value(&self) -> &T {
        match self {
            Cons(_, v, _) => v,
            Nil => panic!("nil"),
        }
    }

    fn left(&self) -> Rc<Self> {
        match self {
            Cons(left, _, _) => left.borrow().clone(),
            Nil => panic!("nil"),
        }
    }

    fn right(&self) -> Rc<Self> {
        match self {
            Cons(_, _, right) => right.borrow().clone(),
            Nil => panic!("nil"),
        }
    }

    fn set_left(&self, other: &Rc<Self>) {
        match self {
            Cons(left, _, _) => *left.borrow_mut() = Rc::clone(other),
            Nil => panic!("nil"),
        }
    }

    fn set_right(&self, other: &Rc<Self>) {
        match self {
            Cons(_, _, right) => *right.borrow_mut() = Rc::clone(other),
            Nil => panic!("nil"),
        }
    }
}

#[derive(Debug)]
pub struct Ring(Rc<DeList<i64>>);

impl Ring {
    pub fn new() -> Self {
        let root = Rc::new(DeList::new(0));
        root.set_left(&root);
        root.set_right(&root);
        Ring(root)
    }

    pub fn insert(&mut self, value: i64) {
        let new = Rc::new(DeList::new(value));
        let left = self.0.left();

        new.set_left(&left);
        left.set_right(&new);

        new.set_right(&self.0);
        self.0.set_left(&new);

        self.rotate_left(1);
    }

    pub fn remove(&mut self) -> i64 {
        let left = self.0.left();
        let right = self.0.right();

        let value = *self.0.value();

        left.set_right(&right);
        right.set_left(&left);

        *self = Ring(right);

        value
    }

    pub fn rotate_left(&mut self, n: usize) {
        for _ in 0..n {
            *self = Ring(self.0.left());
        }
    }

    pub fn rotate_right(&mut self, n: usize) {
        for _ in 0..n {
            *self = Ring(self.0.right());
        }
    }
}
