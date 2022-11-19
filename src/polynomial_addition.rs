use crate::read_values::to_int32;
use crate::read_values::to_usize;

struct Node
{
    coeff:i32,
    pow:i32
}

pub fn polynomial_addition_main()
{
    println!("Enter the values of the first polynomial:");
    let polynomial1 = read_polynomial();

    println!("Enter the values of the second polynomial:");
    let polynomial2 = read_polynomial();

    let result = polynomial_addition(&polynomial1, &polynomial2);
    println!("The sum of the two polynomials is:");
    print_polynomial(&result);
}

fn read_polynomial() -> Vec<Node>
{
    print!("Enter the number of terms: ");
    let mut vector:Vec<Node> = Vec::new();
    let size:usize = to_usize();
    for x in 0..size
    {
        print!("Enter the power of term {}: ", x);
        let pow = to_int32();
        print!("Enter the coefficient of term {}: ", x);
        let coeff = to_int32();
        let new_node:Node = Node{coeff, pow};
        vector.push(new_node);
    }
    vector
}

fn polynomial_addition(polynomial1:&Vec<Node>, polynomial2:&Vec<Node>) -> Vec<Node>
{
    let mut result = Vec::<Node>::new();
    let size1 = polynomial1.len();
    let size2 = polynomial2.len();

    let mut x:usize = 0;
    let mut y:usize = 0;
    while x < size1 && y < size2
    {

        if polynomial1[x].pow == polynomial2[y].pow
        {
            let new_node = Node{coeff:(polynomial1[x].coeff + polynomial2[y].coeff), pow:polynomial1[x].pow};
            result.push(new_node);
            x += 1;
            y += 1;
        }
        else if polynomial1[x].pow > polynomial2[y].pow
        {
            let new_node = Node{coeff:polynomial1[x].coeff, pow:polynomial1[x].pow};
            result.push(new_node);
            x += 1;
        }
        else if polynomial1[x].pow < polynomial2[y].pow
        {
            let new_node = Node{coeff:polynomial2[y].coeff, pow:polynomial2[y].pow};
            result.push(new_node);
            y += 1;
        }
    }
    while x < size1
    {
        let new_node = Node{coeff:polynomial1[x].coeff, pow:polynomial1[x].pow};
        result.push(new_node);
        x += 1;
    }

    while y < size2
    {
        let new_node = Node{coeff:polynomial2[y].coeff, pow:polynomial2[y].pow};
        result.push(new_node);
        y += 1;
    }
    result
}

fn print_polynomial(vector:&Vec<Node>)
{
    for x in 0..vector.len()
    {
        if x != vector.len() - 1
        {
            print!("{}x^{} + ", vector[x].coeff, vector[x].pow);
        }
        else
        {
            println!("{}x^{}", vector[x].coeff, vector[x].pow);
        }
    }
}