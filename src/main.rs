use std::io;
use std::io::Write;

const INTRO: &str = r#"
    This program implements the 'Collatz Conjecture' https://en.wikipedia.org/wiki/Collatz_conjecture

    For any positive number, applying the following rules will always result in reaching the number
    1 in a finite number of steps:
    For each number,
    * if it is odd, multiply by 3 and add 1.
    * If it is even, divide by 2.

    This program will output how many times the rules are applied to reach the number 1.
    For example, if you enter 12, then the program will print 9.

    The program will loop forever. To exit, press <ctrl>-c.
    "#;

fn main() {
    println!("{}", INTRO);

    loop {
        print!("Enter a starting number (greater than 1): ");
        let _ = io::stdout().flush();
        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(mut value) = input.trim().parse::<u32>() {
            if value > 1 {
                let mut count = 0;
                print!("Starting at {value} ... ");
                while value > 1 {
                    if value % 2 == 0 {
                        value /= 2;
                    }
                    else {
                        value = value * 3 + 1;
                    }

                    count += 1;
                }

                println!("it took {count} iterations to reach 1");
            }
        }
    }
}
