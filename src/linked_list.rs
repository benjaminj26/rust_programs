use std::rc::Rc;
use std::cell::RefCell;
use crate::read_values::to_int32;
// use std::iter::Iterator;

#[derive(Clone)]
struct Node
{
    value:i32,
    next:Option<Rc<RefCell<Node>>>,
    prev:Option<Rc<RefCell<Node>>>
}

impl Node
{
    fn new(value:i32) -> Rc<RefCell<Node>>
    {
        let node = Node{value, next:None, prev:None};
        let new_value = Rc::new(RefCell::new(node));
        new_value
    }
}

#[derive(Clone)]
struct LinkedList
{
    head:Option<Rc<RefCell<Node>>>,
    tail:Option<Rc<RefCell<Node>>>,
    current_value:Option<Rc<RefCell<Node>>>
}

impl LinkedList
{
    fn push_back(&mut self)
    {
        println!("Enter the data:");
        let data = to_int32();
        let new_node = Node::new(data);
        let temp_tail = Some(new_node.clone());
        match self.tail
        {
            Some(ref mut x) =>
            {
                new_node.borrow_mut().prev = Some(x.clone());
                x.borrow_mut().next = Some(new_node);
            },

            None =>
            {
                self.current_value = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        self.tail = temp_tail;
    }

    fn pop_back(&mut self)
    {
        let mut temp_tail:Option<Rc<RefCell<Node>>> = None;
        match self.tail
        {
            None =>
            {
                if let None = self.head
                {
                    println!("The list is empty");
                }
                else
                {
                    self.head = None;
                    self.current_value = None;
                }
            },
            Some(ref mut x) => 
            {
                println!("{} is deleted from the list", x.borrow().value);
                temp_tail = x.borrow().prev.clone();
                self.tail.take();
            }
        }

        match temp_tail
        {
            Some(ref mut x) =>
            {
                x.borrow_mut().next = None;
            },

            None =>
            {
                // println!("Something is wrong");
            }
        }
        self.tail = temp_tail;
    }

    fn display(&mut self)
    {
        let mut temp_value:Option<Rc<RefCell<Node>>>;
        loop
        {
            match self.current_value
            {
                None =>
                {
                    if let None = self.head
                    {
                        println!("The list is empty");
                        break;
                    }
                    else
                    {
                        println!();
                        break;
                    }
                },

                Some(ref x) =>
                {
                    print!("{} ", x.borrow().value);
                    temp_value = x.borrow().next.clone();
                }
            }
            self.current_value = temp_value;
        }
        self.current_value = self.head.clone();
    }
}

// impl Copy for Linked_List{}

// impl Clone for LinkedList
// {
//     fn clone(&self) -> LinkedList
//     {
//         head
//     }
// }

// impl Iterator for LinkedList
// {
//     type Item = Node;
//     fn next(&mut self) -> Option<Self::Item>
//     {
//         let temp1 = self.current_value.clone();
//         let temp2:Option<Rc<RefCell<Node>>>;
//         match self.current_value
//         {
//             Some(ref mut x) =>
//             {
//                 temp2 = x.borrow().next.clone();
//             },
//             None =>
//             {
//                 temp2 = None;
//             }
//         }
//         self.current_value = temp2;
//         match temp1
//         {
//             Some(ref x) =>
//             {
//                 return Some(x.borrow().clone());
//             },

//             None =>
//             {
//                 return None;
//             }
//         }
//     }
// }

pub fn linked_list_main()
{
    let mut linked_list = LinkedList{head:None, tail:None, current_value:None};
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
                linked_list.push_back();
            }

            2 =>
            {
                linked_list.pop_back();
            }

            3 =>
            {
                linked_list.display();
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

