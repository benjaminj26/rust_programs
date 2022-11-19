use std::cell::RefCell;
use std::rc::Rc;
use crate::read_values::to_int32;

#[derive(Clone)]
struct Node
{
	value:i32,
	left:Option<Rc<RefCell<Node>>>,
	right:Option<Rc<RefCell<Node>>>
}

impl Node
{
	fn new() -> Option<Rc<RefCell<Node>>>
	{
		print!("\nEnter the new value: ");
		let value = to_int32();

		let new_node = Node{
			value,
			left:None,
			right:None
		};
		return Some(Rc::new(RefCell::new(new_node)));
	}
}

#[derive(Clone)]
struct BinaryTree
{
	head:Option<Rc<RefCell<Node>>>,
	length:usize
}

enum Status
{
	Inserted,
	NotInserted,
	NotFound
}

impl BinaryTree
{	
	fn traverse(
		node:Option<Rc<RefCell<Node>>>, 
		value:i32, 
		new_node: Option<Rc<RefCell<Node>>>
	) -> Status
	{
		match node
		{
			Some(ref x) =>
			{
				let mut temp = x.borrow_mut();
				if temp.value == value
				{
					if temp.left.is_none() && temp.right.is_none()
					{
						println!("\n1.Insert at left");
						println!("2.Insert at right");
						print!("Enter your choice: ");
						let choice = to_int32();

						if choice == 1
						{
							temp.left = new_node.clone();
						}
						else if choice == 2
						{
							temp.right = new_node.clone();
						}
						else
						{
							println!("\nInvalid Input!!");
							println!("The value {} has not been inserted", value);
							return Status::NotInserted;
						}
						return Status::Inserted;
					}

					else if temp.left.is_none() && temp.right.is_some()
					{
						temp.left = new_node.clone();
						return Status::Inserted;
					}
					else if temp.left.is_some() && temp.right.is_none()
					{
						temp.right = new_node.clone();
						return Status::Inserted;
					}
					else
					{
						println!("\nThe specified node has two child nodes");
						return Status::NotInserted;
					}
				}
				else
				{
					let mut status:Status;
					status = BinaryTree::traverse(temp.left.clone(), value, new_node.clone());
					if let Status::Inserted = status
					{
						return status;
					}
					status = BinaryTree::traverse(temp.right.clone(), value, new_node);
					return status;
				}
			},

			None =>
			{
				return Status::NotFound
			}
		}
	}

	fn insert(&mut self, new_node:Option<Rc<RefCell<Node>>>)
	{

		let temp = self.head.clone();
		match temp
		{
			Some(ref _x) =>
			{
				print!("\nEnter the value of the parent node: ");
				let value = to_int32();
				let status = BinaryTree::traverse(temp.clone(), value, new_node.clone());

				if let Status::Inserted = status
				{
					self.length += 1;
				}
				else if let Status::NotFound = status
				{
					println!("\nThe specified node is not found!!");
				}
			},

			None =>
			{
				self.head = new_node;
			}
		}
	}

	fn display(root:Option<Rc<RefCell<Node>>>)
	{
		match root
		{
			Some(ref x) =>
			{
				let temp = x.borrow();
				let value = temp.value;
				print!("{} ", value);
				BinaryTree::display(temp.left.clone());
				BinaryTree::display(temp.right.clone());
			},

			None =>
			{
				return;
			}
		}
	}
}

pub fn binary_tree_main()
{
	let mut binary_tree = BinaryTree{
		head:None,
		length:0
	};

	loop
	{
		println!("1.Insert a new node");
		println!("2.Display all the nodes");
		println!("3.Exit");
		print!("Enter your choice: ");
		
		let choice = to_int32();

		if choice == 1
		{
			let new_node = Node::new();
			binary_tree.insert(new_node);
		}
		else if choice == 2
		{
			println!("\nThe contents of the Binary Tree are:");
			BinaryTree::display(binary_tree.head.clone());
			println!();
		}
		else if choice == 3
		{
			break;
		}
		else
		{
			println!("Invalid Input!!!");
		}
		println!();
	}
}