use std::io;

pub fn add_ints() 
{
    print!("Enter the first number: ");
    let x = read_int32();
    print!("Enter the second number: ");
    let y = read_int32();
    if x != -1 && y != -1
    {
        let sum = x+y;
        println!("The sum of {} and {} is {}", x, y, sum);
    }
    else
    {
        println!("Invalid input");
    }
}

fn read_int32() -> i32 
{
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Unable to read an input from the user");
    let x = x.trim();
    let x = match x.parse::<i32>()
    {
        Ok(x) => x,
        Err(..) => -1
    };
    x
}