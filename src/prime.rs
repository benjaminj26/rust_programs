use std::io;

pub fn prime()
{
    let mut flag:i32 = 0;
    println!("Enter a number: ");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let number:i32 = str.trim().parse().expect("Unable to parse");
    for x in 2..(f32::sqrt(number as f32) as i32 + 1)
    {
        if number % x == 0
        {
            flag = 1;
            break;
        }
    }
    if flag == 0
    {
        println!("{} is a prime number", number);
    }
    else
    {
        println!("{} is not a prime number", number);
    }
}