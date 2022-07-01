use std::rc::Rc;
use std::cell::RefCell;
use crate::read_values::to_int32;
use std::iter::Iterator;

struct Node
{
    value:i32,
    next:Rc<RefCell<Option<Node>>>
}

impl Node
{
    fn new(value:i32) -> Rc<RefCell<Option<Node>>>
    {
        let new_value = Rc::new(RefCell::new(Some(Node{value, next:Rc::new(RefCell::new(None))})));
        new_value
    }
}

// impl Copy for Node{}

impl Clone for Node
{
    fn clone(&self) -> Node
    {
        let node = Node{value:self.value, next:Rc::clone(&self.next)};
        node
    }
}


struct LinkedList
{
    head:Rc<RefCell<Option<Node>>>,
    tail:Rc<RefCell<Option<Node>>>
}

impl LinkedList
{
    fn push_back(&mut self,value:i32)
    {
        println!("Enter the data:");
        let data = to_int32();
        let new_node = Node::new(data);
        let something = self.tail.take();
        match something
        {
            None =>
            {
                self.head = new_node;
                self.tail = Rc::clone(&self.head);
            },

            Some(mut x) =>
            {
                x.next = new_node;
                self.tail = Rc::clone(&x.next);
            }
        }
    }

    fn pop_front(&mut self)
    {
        let something = self.head.take();
        match something
        {
            None =>
            {
                self.head.replace(None);
            },
            Some(x) => 
            {
                self.head = x.next;
            }
        }
    }
}

// impl Copy for Linked_List{}

impl Clone for LinkedList
{
    fn clone(&self) -> Linked_List
    {
        head
    }
}

impl Iterator for Linked_List
{
    type Item = Rc<RefCell<Option<Node>>>;
    fn next(&mut self) -> Option<Self::Item>
    {
        Some(self.head)
    }
}

pub fn linked_list_main()
{
    let mut head:Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    let mut tail:Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    loop
    {
        println!(
            "
                1.Insert new node
                2.Delete a node from the beginning
                3.Display all the nodes
                4.Exit
                Enter your choice:
            "
        );
        let choice = to_int32();
        match choice
        {
            1 =>
            {
                
            }

            2 =>
            {
                
            }

            3 =>
            {
                
            }

            4 =>
            {
                break;
            }

            _ =>
            {
                println!("Invalid Input!!");
            }   
        }
    }

}

