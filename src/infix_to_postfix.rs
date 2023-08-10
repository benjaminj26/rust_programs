/*
    This is a program to convert an infix expression to postfix
    expression using stack, written in Rust

    Author : Benjamin Joseph
    Date : 29-04-2022
*/

use std::io;

pub fn infix_to_postfix() {
    println!("Enter an expression:");
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Unable to read from stdin");
    let mut vector: Vec<char> = Vec::new();
    let str = str.trim();
    for x in str.chars() {
        let top: usize;
        let priority_top: i8;
        if vector.len() != 0 {
            top = vector.len() - 1;
            priority_top = priority(vector[top]);
        } else {
            top = 0;
            priority_top = 0;
        }
        let priority_element = priority(x);
        if priority_element == 0 {
            print!("{}", x);
        } else if vector.len() == 0 || priority_top < priority_element || x == '(' {
            if priority_element != -1 {
                vector.push(x);
            }
        } else {
            if x == ')' {
                while vector.len() > 0 && vector[vector.len() - 1] != '(' {
                    let element = vector[top];
                    vector.pop();
                    if element != '(' {
                        print!("{}", element);
                    }
                }
                vector.pop();
            } else {
                while vector.len() > 0 && priority_element <= priority(vector[vector.len() - 1]) {
                    let element = vector[vector.len() - 1];
                    vector.pop();
                    print!("{}", element);
                }
                vector.push(x);
            }
        }
    }

    while vector.len() > 0 {
        let element = vector[vector.len() - 1];
        vector.pop();
        print!("{}", element);
    }
    println!();
}

fn priority(ch: char) -> i8 {
    if ch == ' ' {
        return -1;
    } else if ch == '(' || ch == ')' {
        return 1;
    } else if ch == '+' || ch == '-' {
        return 2;
    } else if ch == '*' || ch == '/' {
        return 3;
    } else if ch == '^' {
        return 4;
    } else {
        return 0;
    }
}
