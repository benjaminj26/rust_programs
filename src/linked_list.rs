// use std::rc::Rc;
use crate::read_values::to_int32;

struct Node
{
    data:i32,
    next:Option<Box<Node>>
}

impl Node
{
    fn create_node(data:i32) -> Box<Node>
    {
        let new_node = Box::new(Node{data,next:None});
        new_node
    }
    
    fn append(node:Option<Box<Node>>, head:&mut Option<Box<Node>>, tail:&mut Option<Box<Node>>)
    {

    }
}

pub fn linked_list_main()
{
    let mut head:Option<Box<Node>> = None;
    let mut tail:Option<Box<Node>> = None;
    loop
    {
        println!(
            "
                1.Insert new node
                2.Delete a node from the end
                3.Delete a node from the beginning
                4.Display all the nodes
                5.Exit
                Enter your choice:
            "
        );
        let choice = to_int32();
        match choice
        {
            1 => {
                println!("Enter the data:");
                let data = to_int32();
                let new_node = create_node(data);
                if head.is_none() && tail.is_none()
                {
                    head = Some(Box::clone(&new_node));
                    tail = Some(new_node);
                }
                else
                {
                    let temp = tail.unwrap();
                }
            }
            _ => {
                println!("Invalid Input!!");
            }   
        }
    }

}

