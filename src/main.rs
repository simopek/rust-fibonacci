use std::io;

const NUMBERS_ARRAY_SIZE: usize = 50;

fn main() {
    // we init an array with 1000 elements; all elements are set to -1
    let mut numbers = [0; NUMBERS_ARRAY_SIZE];
    numbers[0] = 0;
    numbers[1] = 1;

    loop {
        println!("Which n-th Fibonacci number do you want to know? (q to quit)");

        let nth_num = match read_or_quit() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!(
            "#{} Fibonacci number is {}",
            nth_num,
            fibonacci(&mut numbers, nth_num)
        );
    }
}

fn read_or_quit() -> Result<usize, String> {

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failure reading input");

    let nth_num: usize = match line.trim().parse() {
        Ok(num) => {
            if num > NUMBERS_ARRAY_SIZE {
                println!(

                );
                return Err(format!("we cannot compute the Fibonacci number from #{} onwards",
                           NUMBERS_ARRAY_SIZE + 1));
            } else if num <= 0 {
                return Err("not a valid number".to_string())
            }
            num
        }
        Err(_) => match line.trim().to_lowercase().as_str() {
            "q" => std::process::exit(0),
            _ => {
                return Err("not a number!".to_string());
            }
        },
    };

    return Ok(nth_num);
}

fn fibonacci(numbers: &mut [u128], nth_num: usize) -> u128 {
    // if the i-th element is 0 and we are not considering the first 2 items,
    // that means we still have to compute the Fibonacci number
    if nth_num > 2 && numbers[nth_num - 1] == 0 {
        for idx in 2..nth_num {
            numbers[idx] = numbers[idx - 1] + numbers[idx - 2]
        }
    }
    return numbers[nth_num - 1];
}
