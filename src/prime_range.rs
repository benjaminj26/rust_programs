use std::io;
use std::f32;

pub fn prime_range()
{
    let mut str = String::new();
    println!("Enter the Upper limit:");
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let upper_limit:i32 = str.trim().parse().expect("Unable to parse");

    let mut flag:i32;
    let mut x:i32;
    let mut num:i32 = 0;

    println!("\nThe prime numbers are:");

    while num <= upper_limit
    {
        flag = 0;
        x = 2;
        while x <= f32::sqrt(num as f32) as i32
        {
            if num % x == 0
            {
                flag = 1;
                break;
            }
            x += 1;
        }
        if flag == 0
        {
            println!("{}", num);
        }
        num += 1;
    }
}