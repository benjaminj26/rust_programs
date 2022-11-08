use std::vec;

pub fn closure()
{
	// let x = |num:i32|{
	// 	for x in 0..num
	// 	{
	// 		println!("{}", num + x);
	// 	}
	// };

	// test(x, 30);

	let vector:Vec<i32> = vec![10, 20, 30, 40, 50];

	let y:Vec<i32> = vector.iter().map(|num| {
		return num + 15;
	}).collect();

	for z in y
	{
		print!("{} ", z);
	}
	println!();
}

// fn test<F>(f:F, arg:i32) 
// where F:Fn(i32)
// {
// 	f(arg);
// }