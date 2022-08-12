use std::f32;
use std::env;

pub fn prime_range()
{
    let args:Vec<String> = env::args().collect();
    let upper_limit:i32;
    if args.len() <= 1
    {
        println!("Too few arguments provided");
    }
    else
    {
        upper_limit = args[1].parse().expect("Unable to parse");
        let mut flag:i32;

        println!("\nThe prime numbers are:");

        for num in 0..upper_limit + 1
        {
            flag = 0;
            for x in 2..(f32::sqrt(num as f32) as i32 + 1)
            {
                if num % x == 0
                {
                    flag = 1;
                    break;
                }
            }
            if flag == 0
            {
                println!("{}", num);
            }
        }
    }
}
