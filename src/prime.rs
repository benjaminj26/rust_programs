use crate::read_values::to_int32;

pub fn prime() {
    let mut flag: i32 = 0;
    print!("Enter a number: ");
    let number = to_int32();
    for x in 2..(f32::sqrt(number as f32) as i32 + 1) {
        if number % x == 0 {
            flag = 1;
            break;
        }
    }
    if flag == 0 {
        println!("{} is a prime number", number);
    } else {
        println!("{} is not a prime number", number);
    }
}
