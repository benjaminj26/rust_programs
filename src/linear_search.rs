use crate::read_values::to_int32;
use crate::read_values::to_usize;

pub fn linear_search_main()
{
    print!("Enter the size of the vector: ");
    let size = to_usize();

    let mut vector:Vec<i32> = Vec::new();
    for x in 0..size
    {
        print!("Enter the element at location {}: ", x);
        vector.push(to_int32());
    }
    print!("Enter the element to be searched: ");
    let search = to_int32();

    linear_search(&mut vector, search);
}

fn linear_search(vector:&Vec<i32>, search:i32)
{
    let mut flag = 0;
    for x in 0..vector.len()
    {
        if vector[x] == search
        {
            flag = 1;
            println!("{} found at location {}", search, x);
            break;
        }
    }

    if flag == 0
    {
        println!("Element not found!!");
    }
}
