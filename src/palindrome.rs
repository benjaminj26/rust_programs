use crate::read_values::read_string;

pub fn palindrome()
{
    println!("Enter a string:");
    let str:String = read_string();
    let str = str.trim();
    let mut rev_string = String::new();
    let _length = str.len();
    for x in str.chars().rev()
    {
        rev_string.push(x);
    }
    println!("The reverse of {} is {}", str, rev_string);
    if rev_string.eq(str)
    {
        println!("{} is a palindrome", str);
    }
    else
    {
        println!("{} is not a palindrome", str);
    }
}
