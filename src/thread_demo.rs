use std::thread;

pub fn thread_demo()
{
    let mut thread_vector = Vec::new();

    for x in 0..8
    {
        let t = thread::spawn(move || {
            for num in 0..(100000 * (x % 2) + (x * 18))
            {
                for x in 2..(f32::sqrt(num as f32) as i32 + 1)
                {
                    if num % x == 0
                    {
                        break;
                    }
                }
            }
            println!("Thread {} Done", x + 1);
        });

        thread_vector.push(t);
    }

    for x in thread_vector
    {
        x.join().unwrap();
    }

	println!("All the threads have finished execution");
}