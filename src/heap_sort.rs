use crate::read_values::to_int32;
use crate::read_values::to_usize;
//use crate::read_values::to_uint8;

pub fn heap_sort_main()
{
    println!("Enter the size of the vector:");
    let size:usize = to_usize();
    let mut vector:Vec<i32> = Vec::new();

    println!("Enter the contents of the vector:");

    for x in 0..size
    {
        println!("Enter the value at index {}:", x);
        vector.push(to_int32());
    }

    println!("The vector before sorting is:");
    print_vector(&vector);

    heap_sort(&mut vector);
    println!("The vector after sorting is:");
    print_vector(&vector);

}

fn heap_sort(vector:&mut Vec<i32>)
{
    let size = vector.len();

    for x in (0..(size/2)).rev()
    {
        heapify(vector, x, size);
    }

    for x in (0..size).rev()
    {
        swap_values(vector, x, 0);
        heapify(vector, 0, x);
    }
}

fn heapify(vector:&mut Vec<i32>, index:usize, size:usize)
{
    let mut largest = index;
    let left = index * 2 + 1;
    let right = index * 2 + 2;
    if left < size && vector[left] > vector[largest]
    {
        largest = left;
    }
    
    if right < size &&  vector[right] > vector[largest]
    {
        largest = right;
    }

    if largest != index
    {
        swap_values(vector, index, largest);
        heapify(vector, largest, size);
    }
}

fn swap_values(vector:&mut Vec<i32>, index1:usize, index2:usize)
{
    let temp = vector[index1];
    vector[index1] = vector[index2];
    vector[index2] = temp;
}

fn print_vector(vector:&Vec<i32>)
{
    for x in vector
    {
        print!("{} ", x);
    }
    println!();
}
