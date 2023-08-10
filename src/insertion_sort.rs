use crate::read_values::to_int32;
use crate::read_values::to_usize;

pub fn insertion_sort_main() {
    let mut vector: Vec<i32> = Vec::new();
    print!("Enter the size of the vector: ");
    let size = to_usize();

    for x in 0..size {
        print!("Enter the value of element {}: ", x);
        vector.push(to_int32());
    }
    insertion_sort(&mut vector);
    println!("The sorted vector is:");

    for x in vector {
        print!("{} ", x);
    }
    println!();
}

fn insertion_sort(vector: &mut Vec<i32>) {
    let size = vector.len();
    for x in 1..size {
        let key = vector[x];
        let mut y = x - 1;
        let mut flag = 0;

        while key < vector[y] {
            vector[y + 1] = vector[y];
            if y == 0 {
                vector[y] = key;
                flag = 1;
                break;
            } else {
                y -= 1;
            }
        }
        if flag == 0 {
            vector[y + 1] = key;
        }
    }
}
