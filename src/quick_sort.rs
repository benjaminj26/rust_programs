use crate::read_values::to_int32;
use crate::read_values::to_usize;

pub fn quick_sort_main() {
    print!("Enter the size of the vector: ");
    let size = to_usize();
    let mut vector: Vec<i32> = Vec::new();

    for x in 0..size {
        print!("Enter the element {}: ", x + 1);
        vector.push(to_int32());
    }
    quick_sort(&mut vector, 0, size - 1);
    println!("The vector after sorting is:");
    for x in vector {
        print!("{} ", x);
    }
    println!();
}

fn quick_sort(vector: &mut Vec<i32>, low: usize, upp: usize) {
    if low < upp {
        let location = partition(vector, low, upp);
        if location <= 0 {
            quick_sort(vector, low, location);
        } else {
            quick_sort(vector, low, location - 1);
        }
        quick_sort(vector, location + 1, upp);
    }
}

fn partition(vector: &mut Vec<i32>, low: usize, upp: usize) -> usize {
    let mut start = low + 1;
    let mut end = upp;
    let pivot = vector[low];
    while start < end {
        while start < upp && vector[start] <= pivot {
            start += 1;
        }

        while end >= low && vector[end] > pivot {
            if end == 0 {
                break;
            } else {
                end -= 1;
            }
        }

        if start > upp {
            start = upp;
        }

        if end < low {
            end = low;
        }

        if start < end {
            let temp = vector[start];
            vector[start] = vector[end];
            vector[end] = temp;
        }
    }

    if start == end && vector[end] <= pivot {
        let temp = vector[low];
        vector[low] = vector[end];
        vector[end] = temp;
    } else if start > end {
        let temp = vector[low];
        vector[low] = vector[end];
        vector[end] = temp;
    }

    return end;
}
