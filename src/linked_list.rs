#![allow(dead_code)]

use crate::read_values::to_int32;
use crate::read_values::to_usize;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
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
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
}

impl LinkedList {
    fn new() -> LinkedList {
        return LinkedList {
            head: None,
            tail: None,
            len: 0,
        };
    }

    fn push_back(&mut self) {
        print!("Enter the value: ");
        let value = to_int32();
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

    fn push_front(&mut self) {
        print!("Enter the value: ");
        let value = to_int32();
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

    fn insert_at_index(&mut self) {
        print!("Enter the index: ");
        let index = to_usize();
        if index > self.len + 1 {
            println!("Index is out of bounds")
        } else if index == self.len + 1 {
            self.push_back();
        } else if index == 0 {
            self.push_front();
        } else {
            print!("Enter the value to be inserted: ");
            let value = to_int32();
            let new_node = Node::new(value);
            let mut iterator: Option<Rc<RefCell<Node>>> = self.head.clone();
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

    fn pop_back(&mut self) {
        let mut temp_tail: Option<Rc<RefCell<Node>>> = None;
        match self.tail {
            None => {
                println!("The list is empty");
            }
            Some(ref mut x) => {
                println!("{} is deleted from the list", x.borrow().value);
                temp_tail = x.borrow().prev.clone();
                self.len -= 1;
                self.tail.take();
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
    }

    fn pop_front(&mut self) {
        let mut temp: Option<Rc<RefCell<Node>>> = None;
        match self.head {
            None => {
                println!("The list is empty");
            }

            Some(ref mut x) => {
                println!("{} is deleted", x.borrow().value);
                temp = x.borrow().next.clone();
                self.len -= 1;
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
    }

    fn delete_from_index(&mut self) {
        print!("Enter the index to delete from: ");
        let index = to_usize();

        if index >= self.len {
            println!("Index is out of bounds");
        } else if index == 0 {
            self.pop_front();
        } else if index == self.len {
            self.pop_back();
        } else {
            let mut iterator = self.head.clone();
            for _ in 0..(index - 1) {
                if let Some(ref x) = iterator.clone() {
                    iterator = x.borrow().next.clone();
                }
            }
            let iterator = iterator.unwrap();
            println!("{} is deleted", iterator.borrow().value);
            if let Some(ref x) = iterator.borrow().prev {
                x.borrow_mut().next = iterator.borrow().next.clone();
            }

            if let Some(ref x) = iterator.borrow().next {
                x.borrow_mut().prev = iterator.borrow().prev.clone();
            }
            self.len -= 1;
        }
    }

    fn iter<'a>(&self) -> Iter<'a> {
        return Iter {
            current_value: self.head.clone(),
            phantom: PhantomData,
        };
    }

    fn iter_mut<'a>(&mut self) -> IterMut<'a> {
        return IterMut {
            current_value: self.head.clone(),
            phantom: PhantomData,
        };
    }
}

struct Iter<'a> {
    current_value: Option<Rc<RefCell<Node>>>,
    phantom: PhantomData<&'a i32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value.is_some() {
            let unwrapped = self.current_value.clone().unwrap();
            self.current_value = unwrapped.borrow().next.clone();

            let raw_pointer: *mut Node = unwrapped.as_ptr();
            unsafe {
                return Some(&(*raw_pointer).value);
            }
        } else {
            return None;
        }
    }
}

struct IterMut<'a> {
    current_value: Option<Rc<RefCell<Node>>>,
    phantom: PhantomData<&'a i32>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value.is_some() {
            let unwrapped = self.current_value.clone().unwrap();
            self.current_value = unwrapped.borrow().next.clone();

            let raw_pointer: *mut Node = unwrapped.as_ptr();
            unsafe {
                return Some(&mut (*raw_pointer).value);
            }
        } else {
            return None;
        }
    }
}

pub fn linked_list_main() {
    let mut linked_list = LinkedList::new();

    loop {
        println!("1.Insert a node at the end\n2.Delete the node at the end");
        println!("3.Insert a node at the beginning\n4.Delete the node at the beginning");
        println!("5.Insert at an index\n6.Delete from an index");
        println!("7.Display all the nodes\n8.Display the element at an index");
        print!("9.Display the length of the list\n10.Exit\nEnter your choice: ");
        let choice = to_int32();
        match choice {
            1 => {
                linked_list.push_back();
            }

            2 => {
                linked_list.pop_back();
            }

            3 => {
                linked_list.push_front();
            }

            4 => {
                linked_list.pop_front();
            }

            5 => {
                linked_list.insert_at_index();
            }

            6 => {
                linked_list.delete_from_index();
            }

            7 => {
                println!("The elements of the list are:");
                for x in linked_list.iter() {
                    print!("{} ", x);
                }
                println!();
            }

            8 => {
                print!("Enter the index of the element: ");
                let index = to_usize();
                if linked_list.len == 0 {
                    println!("The list is empty");
                } else if index > linked_list.len {
                    println!("Index is out of bounds");
                } else {
                    let mut iterator = linked_list.head.clone().unwrap();
                    for _ in 0..(index - 1) {
                        iterator = iterator.clone().borrow().next.clone().unwrap();
                    }
                    let value = iterator.borrow().value;
                    println!("The element at index {} is {}", index, value);
                }
            }

            9 => {
                println!("The length of the list is {}", linked_list.len);
            }

            10 => {
                break;
            }

            _ => {
                println!("Invalid Input!!");
            }
        }
        println!();
    }
}
