mod linked_list;
use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(89);
    list.push_front(67);
    list.push_back(34);
    list.push_front(22);

    list.insert_at_index(48, 4);
    list.push_back(17);
    list.push_front(59);

    println!("The length of the list is {}", list.len());

    for x in list.iter() {
        print!("{} ", x);
    }
    println!();

    for x in list.iter_mut() {
        *x += 10;
    }

    println!("\nAfter modification:");
    for x in list.iter() {
        print!("{} ", x);
    }
    println!("\n");

    if let Some(x) = list.pop_back() {
        println!("{} is deleted", x);
    }

    if let Some(x) = list.pop_front() {
        println!("{} is deleted", x);
    }

    if let Some(x) = list.delete_from_index(2) {
        println!("{} is deleted", x);
    }

    println!("\nThe list after deletion is:");
    for x in list.iter() {
        print!("{} ", x);
    }
    println!();
}
