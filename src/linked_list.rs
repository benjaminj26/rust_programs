use std::rc::Rc;
use crate::read_values::to_int32;

struct Node
{
    data:i32,
    next:Option<Rc<Node>>
}

pub fn linked_list_main()
{
    let mut head:Option<Rc<Node>> = None;
    let mut tail:Option<Rc<Node>> = None;
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
                head = Some(new_node);
                tail = Rc::clone(&new_node);
            }
        }
    }
}

fn create_node(data:i32) -> Rc<Node>
{
    let new_node = Rc::new(Node{data,next:None});
    new_node
}