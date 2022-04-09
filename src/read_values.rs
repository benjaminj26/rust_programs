use std::io;

pub fn to_int32() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let num:i32 = str.trim().parse().expect("Unable to parse");
    num
}

// pub fn to_uint32() -> u32
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:u32 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_float32() -> f32
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:f32 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_int64() -> i64
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:i64 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_uint64() -> u64
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:u64 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_float64() ->f64
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:f64 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_int16() ->i16
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:i16 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_uint16() ->u16
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:u16 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_int8() -> i8
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:i8 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_uint8() -> u8
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:u8 = str.trim().parse().expect("Unable to parse");
//     num
// }

// pub fn to_size() -> isize
// {
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("Unable to read from stdin");
//     let num:isize = str.trim().parse().expect("Unable to parse");
//     num
// }

pub fn to_usize() -> usize
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let num:usize = str.trim().parse().expect("Unable to parse");
    num
}