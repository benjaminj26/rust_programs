use crate::read_values::to_int32;
use crate::read_values::to_usize;

pub fn bubble_sort()
{
    let mut vector:Vec<i32> = read_vector();

    println!("The vector before sorting is:");
    print_vector(&vector);

    for x in 1..vector.len()
    {
        for y in 0..vector.len()-x
        {
            if vector[y] > vector[y+1]
            {
                let tmp = vector[y];
                vector[y] = vector[y+1];
                vector[y+1] = tmp;
            }
        }
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

    for x in 0..size
    {
        println!("Enter element {}:", x);
        vector.push(to_int32());
    }
    vector
}