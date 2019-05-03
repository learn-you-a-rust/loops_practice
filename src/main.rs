use std::io;

fn fahrenheit_to_celsius_table() {
    println!("FAHRENHEIT ......... CELSIUS");
    for f_degree in -459..213 {
        let c_degree = (f_degree as f64 - 32.0) * (5.0/9.0);
        println!("{} .................... {}", f_degree, c_degree);
    }
}

// this is awkward because i'm not sure how to pass the argument
// initially on the command line
fn fibonacci() -> i32 {
    println!("Which fibonacci number would you like?\n");

    // read input
    let mut fib_seq = String::new();
    io::stdin().read_line(&mut fib_seq).expect("Failed to read input");
    let n: u32 = fib_seq.trim().parse().expect("Invalid choice");

    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut prev = 1;
        let mut num = 0 + 1;
        for _i in 4..(n + 1) {
            let temp = num;
            num = num + prev;
            prev = temp;
        }
        num
    }
}

fn main() {
    //fahrenheit_to_celsius_table();

    let fib_num = fibonacci();
    println!("The fibonacci number is {}", fib_num);
}
