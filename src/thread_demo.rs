use std::{thread, time::Duration};

pub fn thread_demo()
{
	let t1 = thread::spawn(|| {
		for _ in 0..10
		{
			println!("From Spawned thread!");
			thread::sleep(Duration::from_secs(2));
		}
	});

	for _ in 0..10
	{
		println!("From Main thread!");
		thread::sleep(Duration::from_secs(1));
	}
	t1.join().unwrap();
}