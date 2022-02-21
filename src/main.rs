use std::io;

fn main() {
    //Position of the requested Fibonacci number
    let n: u64;
    //vector to save Fibonacci numbers
    let mut stack = vec![0, 1];
    //buffer variable for newly generated fibonacci numbers
    let mut buffer: u64 = 0;
    //variable to save the user input
    let mut input_text = String::new();

    //trim user input and convert it to u32 integer
    println!("----Fibonacci Finder----");
    println!("Enter a number n to get the n-th Fibonacci number.");
    println!("(Due to technical limitations n>94 will result in a crash)");
    println!("Please input n");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line!");
    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(i) => {
            n = i - 1;
        }
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            panic!();
        }
    }
    //Fibonacci number generation
    for _number in 1..n {
        buffer = stack[stack.len() - 1] + stack[stack.len() - 2];
        stack.push(buffer);
    }
    //Output
    println!("----Results----");
    let mut counter: u8 = 1;
    for f in stack {
        println!("F({}): {}", counter, f);
        counter += 1;
    }
}
/*

*/
