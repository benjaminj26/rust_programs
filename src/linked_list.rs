#![allow(dead_code)]
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        let node = Node {
            value,
            next: None,
            prev: None,
        };
        let new_value = Rc::new(RefCell::new(node));
        new_value
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        return LinkedList {
            head: None,
            tail: None,
            len: 0,
        };
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        let temp_tail = Some(new_node.clone());
        match self.tail {
            Some(ref mut x) => {
                new_node.borrow_mut().prev = Some(x.clone());
                x.borrow_mut().next = Some(new_node);
            }

            None => {
                self.head = Some(new_node);
            }
        }
        self.len += 1;
        self.tail = temp_tail;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        let temp = new_node.clone();
        match self.head {
            None => {
                self.tail = Some(temp.clone());
            }

            Some(ref mut x) => {
                new_node.borrow_mut().next = Some(x.clone());
                x.borrow_mut().prev = Some(new_node.clone());
            }
        }
        self.len += 1;
        self.head = Some(new_node);
    }

    pub fn insert_at_index(&mut self, value: T, index: usize) {
        if index > self.len + 1 {
            println!("Index is out of bounds")
        } else if index == self.len + 1 {
            self.push_back(value);
        } else if index == 0 {
            self.push_front(value);
        } else {
            let new_node = Node::new(value);
            let mut iterator = self.head.clone();
            for _ in 0..(index - 2) {
                if let Some(ref mut x) = iterator.clone() {
                    iterator = x.borrow().next.clone();
                }
            }
            if let Some(ref mut x) = iterator {
                {
                    let mut temp = new_node.borrow_mut();
                    temp.next = x.borrow().next.clone();
                    temp.prev = Some(x.clone());
                }
                x.borrow_mut().next = Some(new_node.clone());
            }
            if let Some(ref mut x) = new_node.borrow_mut().next {
                x.borrow_mut().prev = Some(new_node.clone());
            }
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut temp_tail;
        let return_value;
        match self.tail {
            None => {
                println!("The list is empty");
                return None;
            }
            Some(ref mut x) => {
                temp_tail = x.borrow().prev.clone();
                self.len -= 1;
                return_value = self.tail.take().unwrap();
            }
        }

        match temp_tail {
            Some(ref mut x) => {
                x.borrow_mut().next = None;
            }

            None => {
                self.head = None;
            }
        }
        self.tail = temp_tail;
        let return_value = Rc::try_unwrap(return_value);
        match return_value {
            Ok(x) => {
                return Some(x.into_inner().value);
            }, Err(_) => {
                return None;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut temp;
        let return_value;
        match self.head {
            None => {
                println!("The list is empty");
                return None;
            }

            Some(ref mut x) => {
                temp = x.borrow().next.clone();
                self.len -= 1;
                return_value = self.head.take().unwrap();
            }
        }

        match temp {
            None => {
                self.tail = None;
            }

            Some(ref mut x) => {
                x.borrow_mut().prev = None;
            }
        }
        self.head = temp;
        let return_value = Rc::try_unwrap(return_value);
        match return_value {
            Ok(x) => {
                return Some(x.into_inner().value);
            }, Err(_) => {
                return None;
            }
        }
    }

    pub fn delete_from_index(&mut self, index: usize) -> Option<T> {

        if index >= self.len {
            println!("Index is out of bounds");
            return None;
        } else if index == 0 {
            return self.pop_front();
        } else if index == self.len {
            return self.pop_back();
        } else {
            let mut iterator = self.head.clone();
            for _ in 0..(index - 1) {
                if let Some(ref x) = iterator.clone() {
                    iterator = x.borrow().next.clone();
                }
            }
            let iterator = iterator.unwrap();
            if let Some(ref x) = iterator.borrow().prev {
                x.borrow_mut().next = iterator.borrow().next.clone();
            }

            if let Some(ref x) = iterator.borrow().next {
                x.borrow_mut().prev = iterator.borrow().prev.clone();
            }
            self.len -= 1;
            let iterator = Rc::try_unwrap(iterator);
            match iterator {
                Ok(x) => {
                    return Some(x.into_inner().value);
                }, Err(_) => {
                    return None;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn iter<'a>(&self) -> Iter<'a, T> {
        return Iter {
            current_value: self.head.clone(),
            phantom: PhantomData,
        };
    }

    pub fn iter_mut<'a>(&mut self) -> IterMut<'a, T> {
        return IterMut {
            current_value: self.head.clone(),
            phantom: PhantomData,
        };
    }
}

pub struct Iter<'a, T> {
    current_value: Option<Rc<RefCell<Node<T>>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value.is_some() {
            let unwrapped = self.current_value.clone().unwrap();
            self.current_value = unwrapped.borrow().next.clone();

            let raw_pointer: *mut Node<T> = unwrapped.as_ptr();
            unsafe {
                return Some(&(*raw_pointer).value);
            }
        } else {
            return None;
        }
    }
}

pub struct IterMut<'a, T> {
    current_value: Option<Rc<RefCell<Node<T>>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value.is_some() {
            let unwrapped = self.current_value.clone().unwrap();
            self.current_value = unwrapped.borrow().next.clone();

            let raw_pointer: *mut Node<T> = unwrapped.as_ptr();
            unsafe {
                return Some(&mut (*raw_pointer).value);
            }
        } else {
            return None;
        }
    }
}