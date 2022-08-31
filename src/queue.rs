use crate::read_values::to_int32;

pub fn queue_main()
{
	const LENGTH:usize = 5;
	let mut queue:[i32;LENGTH] = [-1; LENGTH];
	let mut front:usize = LENGTH;
	let mut rear:usize = LENGTH;
	
	loop
	{
		println!("1.Enqueue");
		println!("2.Dequeue");
		println!("3.Display The Elements");
		println!("4.Exit");
		println!("Enter your choice:");
		let choice = to_int32();

		match choice
		{
			1 =>
			{
				enqueue(&mut queue, LENGTH, &mut front, &mut rear);
			},

			2 =>
			{
				dequeue(&mut queue, LENGTH, &mut front, &mut rear);
			},

			3 =>
			{
				if front == LENGTH && rear == LENGTH
				{
					println!("The queue is empty!!!");
				}
				else if front == rear + 1
				{
					println!("The queue is empty!!!");
				}
				else
				{
					for x in queue.iter()
					{
						if *x != -1
						{
							print!("{} ", *x);
						}
					}
					println!();
				}
			},

			4 =>
			{
				break;
			},

			_ =>
			{
				println!("Invalid Input!!!");
			}
		}
		println!();
	}
}

fn enqueue(queue:&mut [i32], length:usize, front:&mut usize, rear:&mut usize)
{
	if *front == length && *rear == length
	{
		println!("Enter the value:");
		let value = to_int32();
		*front = 0;
		*rear = 0;
		queue[*rear] = value;
	}
	else if *rear == length - 1
	{
		println!("The queue is full!!");
	}
	else
	{
		println!("Enter the value:");
		let value = to_int32();
		*rear += 1;
		queue[*rear] = value;
	}
}

fn dequeue(queue:&mut [i32], length:usize, front:&mut usize, rear:&mut usize)
{
	if *front == length && *rear == length
	{
		println!("The queue is empty!!!");
	}
	else if *front == *rear + 1
	{
		println!("The queue is empty!!!");
	}
	else if *front == length - 1
	{
		println!("{} is removed from the queue", queue[*front]);
		queue[*front] = -1;
		*front = length;
		*rear = length;
	}
	else
	{
		println!("{} is removed from the queue", queue[*front]);
		queue[*front] = -1;
		*front += 1;	
	}
}