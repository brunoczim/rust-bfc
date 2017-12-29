use std::{
    mem,
};
use std::ops::{
    Deref,
    DerefMut,
};


/// A linked list that always have at least one element.
#[derive(Clone, Debug)]
pub struct HeadedList<T> {
    val: T,
    next: Option<Box<HeadedList<T>>>,
}

impl<T> HeadedList<T> {

    pub fn new(val: T, next: Option<HeadedList<T>>) -> Self {
        HeadedList {
            val,
            next: match next {
                Some(x) => Some(Box::new(x)),
                _ => None,
            }
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut T {
        &mut self.val
    }

    pub fn val_cpy(&self) -> T where T: Copy {
        self.val
    }

    pub fn next(&self) -> Option<&HeadedList<T>> {
        match &self.next {
            &Some(ref x) => Some(x.deref()),
            _ => None,
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut HeadedList<T>> {
        match &mut self.next {
            &mut Some(ref mut x) => Some(x.deref_mut()),
            _ => None,
        }
    }

    pub fn put_next(&mut self, next: Option<HeadedList<T>>) -> Option<HeadedList<T>> {
        match mem::replace(&mut self.next, match next {
            Some(x) => Some(Box::new(x)),
            _ => None,
        }) {
            Some(x) => Some(*x),
            _ => None,
        }
    }

    pub fn take(&mut self) -> Option<T> {
        match self.next.take() {
            Some(node) => {
                let real_node = *node;
                let HeadedList {val, next} = real_node;
                mem::replace(&mut self.next, next);
                Some(mem::replace(&mut self.val, val))
            },
            _ => None,
        }
    }

    pub fn receive(&mut self, val: T) {
        let next = mem::replace(self, Self{
            val,
            next: None,
        });
        self.next = Some(Box::new(next));
    }

    pub fn reclaim(self) -> (T, Option<HeadedList<T>>) {
        match self.next {
            Some(boxed) => (self.val, Some(*boxed)),
            _ => (self.val, None),
        }
    }

    pub fn reclaim_val(self) -> T {
        self.val
    }

}

