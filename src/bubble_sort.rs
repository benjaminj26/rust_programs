use std::io;

pub fn bubble_sort()
{
    let mut vector:Vec<i32> = read_vector();
    let mut x:usize = 1;
    let mut y:usize;

    println!("The vector before sorting is:");
    print_vector(&vector);

    while x < vector.len()
    {
        y=0;
        while y < vector.len()-x
        {
            if vector[y] > vector[y+1]
            {
                let tmp = vector[y];
                vector[y] = vector[y+1];
                vector[y+1] = tmp;
            }
            y+=1;
        }
        x+=1;
    }
    println!("The vector after sorting is:");
    print_vector(&vector);
}

fn print_vector(vector: &Vec<i32>)
{
    for x in vector
    {
        print!("{} ", x);
    }
    println!();
}

fn read_vector() -> Vec<i32>
{
    println!("Enter the size of the array:");
    let size:usize = to_usize();
    let mut vector:Vec<i32> = Vec::new();

    let mut x:usize = 0;
    while x < size
    {
        println!("Enter element {}:", x);
        vector.push(to_int32());
        x += 1;
    }
    vector
}

pub fn to_int32() -> i32
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let num:i32 = str.trim().parse().expect("Unable to parse");
    num
}


pub fn to_usize() -> usize
{
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Unable to read from stdin");
    let num:usize = str.trim().parse().expect("Unable to parse");
    num
}