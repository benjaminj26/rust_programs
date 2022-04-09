use std::io;
use std::f32;

pub fn prime_range()
{
    let mut str = String::new();
    println!("Enter the Upper limit:");
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let upper_limit:i32 = str.trim().parse().expect("Unable to parse");

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