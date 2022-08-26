use crate::read_values::to_int32;

pub fn stack_main()
{
	println!("Enter the size of the stack: ");
	const LENGTH:usize = 5;
	let mut stack:[i32; LENGTH] = [-1; LENGTH];
	let mut top = LENGTH;
	loop
	{
		println!("1.Push");
		println!("2.Pop");
		println!("3.Display");
		println!("4.Exit");
		println!("Enter your choice: ");
		let choice = to_int32();

		match choice
		{
			1 =>
			{
				push(LENGTH, &mut stack, &mut top);
			},

			2 =>
			{
				pop(LENGTH, &mut stack, &mut top)
			},

			3 =>
			{
				if top == LENGTH
				{
					println!("\nThe stack is empty!!");
				}
				display(&stack);
			},

			4 =>
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

fn push(length:usize, stack:&mut [i32], top:&mut usize)
{
	if *top == length - 1
	{
		println!("\nStack Overflow!!");
	}
	else if *top == length
	{
		println!("Enter the value to be pushed into the stack:");
		let value = to_int32();
		*top = 0;
		stack[*top] = value;
		println!("\n{} pushed to stack", value);
	}
	else
	{
		println!("Enter the value to be pushed into the stack:");
		let value = to_int32();
		*top += 1;
		stack[*top] = value;
		println!("\n{} pushed to stack", value);
	}
}

fn pop(length:usize, stack:&mut [i32], top:&mut usize)
{
	if *top == length
	{
		println!("\nStack Underflow!!");
	}
	else if *top == 0
	{
		println!("\n{} is popped from the stack", stack[*top]);
		stack[*top] = -1;
		*top = length;
	}
	else
	{
		println!("\n{} is popped from the stack", stack[*top]);
		stack[*top] = -1;
		*top -= 1;
	}
}

fn display(stack:&[i32])
{
	for x in stack.iter()
	{
		if *x != -1
		{
			print!("{} ", *x);
		}
	}
	println!();
}
