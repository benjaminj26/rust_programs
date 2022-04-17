use crate::read_values::to_usize;
use crate::read_values::to_int32;

pub fn merge_sort_main()
{
    println!("Enter the size of the vector:");
    let size:usize = to_usize();
    let mut vector:Vec<i32> = Vec::new();
    for x in 0..size
    {
        println!("Enter the element {}", x+1);
        vector.push(to_int32());
    }
    merge_sort(&mut vector, 0, size-1);
    println!("The sorted vector is:");
    for x in vector
    {
        print!("{} ", x);
    }
    println!();
}

fn merge_sort(vector:&mut Vec<i32>, low:usize, upp:usize)
{
    if low < upp
    {
        let mid = low + (upp - low)/2;
        merge_sort(vector, low, mid);
        merge_sort(vector, mid+1, upp);
        merge(vector, low, mid, upp);
    }
    else
    {
        return;
    }
}

fn merge(vector:&mut Vec<i32>, low:usize, mid:usize, upp:usize)
{
    let left_size:usize = mid - low + 1;
    let mut left_vector:Vec<i32> = Vec::new();

    let right_size:usize = upp - mid;
    let mut right_vector:Vec<i32> = Vec::new();

    for x in low..(mid + 1)
    {
        left_vector.push(vector[x]);
    }

    for x in (mid+1)..(upp + 1)
    {
        right_vector.push(vector[x]);
    }

    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut k:usize = low;

    while i < left_size && j < right_size
    {
        if left_vector[i] <= right_vector[j]
        {
            vector[k] = left_vector[i];
            i += 1;
            k += 1;
        }
        
        else
        {
            vector[k] = right_vector[j];
            j += 1;
            k += 1;
        }
    }

    while i < left_size
    {
        vector[k] = left_vector[i];
        i += 1;
        k += 1;
    }

    while j < right_size
    {
        vector[k] = right_vector[j];
        j += 1;
        k += 1;
    }
}