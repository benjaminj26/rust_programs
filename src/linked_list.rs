use std::rc::Rc;
use std::cell::RefCell;
use crate::read_values::to_int32;
use crate::read_values::to_usize;

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
        let node = Node
        {
            value,
            next:None,
            prev:None
        };
        let new_value = Rc::new(RefCell::new(node));
        new_value
    }
}

#[derive(Clone)]
struct LinkedList
{
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    current_value: Option<Rc<RefCell<Node>>>,
    len: usize,
    index_table: Vec<Rc<RefCell<Node>>> 
}

impl LinkedList
{
    fn push_back(&mut self)
    {
        println!("Enter the value:");
        let value = to_int32();
        let new_node = Node::new(value);
        let temp_tail = Some(new_node.clone());
        match self.tail
        {
            Some(ref mut x) =>
            {
                new_node.borrow_mut().prev = Some(x.clone());
                self.index_table.push(new_node.clone());
                x.borrow_mut().next = Some(new_node);
                self.len += 1;
            },

            None =>
            {
                self.index_table.push(new_node.clone());
                self.head = Some(new_node);
                self.len += 1;
            }
        }
        self.tail = temp_tail;
    }

    fn push_front(&mut self)
    {
        println!("Enter the value:");
        let value = to_int32();
        let new_node = Node::new(value);
        let temp = new_node.clone();
        let mut temp_index_vec:Vec<Rc<RefCell<Node>>> = Vec::new();
        match self.head
        {
            None =>
            {
                self.tail = Some(temp.clone());
                self.len += 1;
                temp_index_vec.push(new_node.clone());
            },

            Some(ref mut x) =>
            {
                new_node.borrow_mut().next = Some(x.clone());
                x.borrow_mut().prev = Some(new_node.clone());
                self.len += 1;
                temp_index_vec.push(new_node.clone());

                temp_index_vec.append(&mut self.index_table);
            }
        }
        self.head = Some(new_node);
        self.index_table = temp_index_vec;
    }

    fn insert_at_index(&mut self)
    {
        println!("Enter the index:");
        let index = to_usize();
        if index >= self.len
        {
            println!("The entered index is not accessible");
        }
        else
        {
            println!("Enter the value to be inserted:");
            let value = to_int32();
            let new_node = Node::new(value);
            new_node.borrow_mut().prev = self.index_table[index].borrow().prev.clone();
            new_node.borrow_mut().next = Some(self.index_table[index].clone());
            self.index_table[index].borrow_mut().prev = Some(new_node.clone());
            if let Some(ref mut x) = new_node.borrow_mut().prev
            {
                x.borrow_mut().next = Some(new_node.clone());
            }
    
            let mut temp_index_vec:Vec<Rc<RefCell<Node>>> = Vec::new();
            temp_index_vec.extend_from_slice(&self.index_table[..index]);
            temp_index_vec.push(new_node.clone());
            temp_index_vec.extend_from_slice(&self.index_table[index..]);
            
            self.index_table = temp_index_vec;
            self.len += 1;
        }
    }

    fn pop_back(&mut self)
    {
        let mut temp_tail:Option<Rc<RefCell<Node>>> = None;
        match self.tail
        {
            None =>
            {
                println!("The list is empty");
            },
            Some(ref mut x) => 
            {
                println!("{} is deleted from the list", x.borrow().value);
                temp_tail = x.borrow().prev.clone();
                self.index_table.pop();
                self.len -= 1;
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
                self.head = None;
            }
        }
        self.tail = temp_tail;
    }

    fn pop_front(&mut self)
    {
        let mut temp:Option<Rc<RefCell<Node>>> = None; 
        match self.head
        {
            None =>
            {
                println!("The list is empty");
            },

            Some(ref mut x) =>
            {
                println!("{} is deleted", x.borrow().value);
                temp = x.borrow().next.clone();
                self.len -= 1;
                let mut temp_index_vec = Vec::new();
                temp_index_vec.extend_from_slice(&self.index_table[1..]);
                self.index_table = temp_index_vec;
            }
        }

        match temp
        {
            None =>
            {
                self.tail = None;
            },

            Some(ref mut x) =>
            {
                x.borrow_mut().prev = None;
            }
        }
        self.head = temp;
    }

    fn display(&mut self)
    {
        self.current_value = self.head.clone();
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

pub fn linked_list_main()
{
    let mut linked_list = LinkedList
    {
        head:None,
        tail:None,
        current_value:None,
        len:0,
        index_table:Vec::new()
    };
    loop
    {
        println!("1.Insert a node at the end\n2.Delete a node from the end");
        println!("3.Insert a node at the beginning\n4.Delete a node from the end");
        println!("5.Insert at an index\n6.Delete from an index");
        println!("7.Display all the nodes\n8.Display the element at an index");
        println!("9.Display the length of the list\n10.Exit\nEnter your choice:");
        let choice = to_int32();
        match choice
        {
            1 =>
            {
                linked_list.push_back();
            },

            2 =>
            {
                linked_list.pop_back();
            },

            3 =>
            {
                linked_list.push_front();
            },

            4 =>
            {
                linked_list.pop_front();
            },

            5 =>
            {
                linked_list.insert_at_index();
            },

            7 =>
            {
                linked_list.display();
            },

            8 =>
            {
                println!("Enter the index of the element:");
                let index = to_usize();
                if linked_list.len == 0
                {
                    println!("The list is empty");
                }
                else if index >= linked_list.len
                {
                    println!("The entered index value is greater than the length of the list");
                }
                else
                {
                    let value = linked_list.index_table[index].borrow().value;
                    println!("The element at index {} is {}", index, value);
                }
            },

            9 =>
            {
                println!("The length of the list is {}", linked_list.len);
            },

            10 =>
            {
                break;
            },

            _ =>
            {
                println!("Invalid Input!!");
            }   
        }
        println!();
    }
}