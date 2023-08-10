use std::io;

pub fn string_compare() {
    let mut str1: String = String::new();
    let mut str2: String = String::new();
    println!("Enter the first string:");
    io::stdin()
        .read_line(&mut str1)
        .expect("Unable to read line");

    println!("Enter the second string:");
    io::stdin()
        .read_line(&mut str2)
        .expect("Unable to read line");

    if str1.eq_ignore_ascii_case(&str2) == true {
        println!("The two strings are equal");
    } else {
        println!("The two strings are not equal");
    }
}
