use crate::read_values::to_usize;
use crate::read_values::to_int32;

pub fn selection_sort_main()
{
    print!("Enter the size of the vector: ");
    let size = to_usize();

    let mut vector:Vec<i32> = Vec::new();

    for x in 0..size
    {
        print!("Enter the element at index {}: ", x);
        vector.push(to_int32());
    }
    selection_sort(&mut vector);

    println!("The vector after sorting is:");
    for x in vector
    {
        print!("{} ", x);
    }
    println!();
}

fn selection_sort(vector:&mut Vec<i32>)
{
    let size = vector.len();
    for x in 0..size-1
    {
        let mut smallest = x;
        for y in (x+1)..size
        {
            if vector[y] < vector[smallest]
            {
                smallest = y;
            }
        }
        
        let temp = vector[smallest];
        vector[smallest] = vector[x];
        vector[x] = temp;
    }
}
