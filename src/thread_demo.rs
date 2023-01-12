use std::thread;

pub fn thread_demo()
{
    let mut thread_vector = Vec::new();

    let thread_vals:[u32; 8] = [
        2000000, 5000000,
        3000000, 1000000,
        7000000, 4000000,
        8000000, 6000000
    ];

    for x in 0..8
    {
        let t = thread::spawn(move || {
            let upp = thread_vals[x];
            for num in 0..upp
            {
                for y in 2..(f32::sqrt(num as f32) as u32 + 1)
                {
                    if num % y == 0
                    {
                        break;
                    }
                }
            }
            println!("Finished upto {}", upp);
        });

        thread_vector.push(t);
    }

    for x in thread_vector
    {
        x.join().unwrap();
    }

	println!("All the threads have finished execution");
}