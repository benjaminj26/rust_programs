use std::thread::*;
use std::{io, io::BufRead};

pub fn io_lock() {
    let t1 = spawn(|| {
        let stdin = io::stdin();
        let mut string = String::new();
        for _ in 0..3 {
            stdin.read_line(&mut string).expect("Unable to read");
            println!("From Thread 1 You wrote:\n{}", string);
        }
    });

    let t2 = spawn(|| {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        for _ in 0..3 {
            let mut string = String::new();
            handle.read_line(&mut string).expect("Unable to read");
            println!("From Thread 2 You wrote:\n{}", string);
        }
    });

    t1.join().expect("Unable to join thread");
    t2.join().expect("Unable to join thread");
}
