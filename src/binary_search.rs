use crate::read_values::to_usize;
use crate::read_values::to_uint8;
use crate::read_values::to_int32;

pub fn binary_search_main()
{
    println!("Enter the size of the vector:");
    let size:usize = to_usize();
    let mut vector:Vec<i32> = Vec::new();
    for x in 0..size
    {
        println!("Enter the element {}", x);
        vector.push(to_int32());
    }
     
    loop
    {
        println!("1.Binary Search");
        println!("2.Exit");
        println!("Enter your choice:");
        let choice:u8 = to_uint8();
        if choice == 1
        {
            binary_search(&vector, size);
        }
        else if choice == 2
        {
            break;
        }
        else
        {
            println!("\nInvalid Input\n");
        }
    }
}

fn binary_search(vector:&Vec<i32>, size:usize)
{
    let mut low:usize = 0;
    let mut upp:usize = size-1;
    let mut mid:usize = low + (upp - low)/2;
    let mut flag:u8 = 0;

    println!("Enter the element to be searched:");
    let element:i32 = to_int32();

    while mid > 0 && mid < size
    {
        if vector[mid] == element
        {
            flag = 1;
            println!("The element is found at location {}", mid);
            break;
        }
        else if element < vector[mid]
        {
            upp = mid;
            if (upp - low)/2 == 0
            {
                mid -= 1;
                if vector[mid] == element
                {
                    flag = 1;
                    println!("Element found at location {}", mid);
                    break;
                }
            }
            else
            {
                mid = low + (upp - low)/2;
            }
        }
        else if element > vector[mid]
        {
            low = mid;
            if (upp - low)/2 == 0
            {
                mid += 1;
                if vector[mid] == element
                {
                    flag = 1;
                    println!("Element found at location {}", mid);
                    break;
                }
            }
            else
            {
                mid = low + (upp - low)/2;
            }
        }
    }
    if flag == 0
    {
        println!("Element not found");
    }
}
