use std::io;
pub const EXIT_CODE: &str = "exit";
const TRY:u32 = 3;

pub fn input_string() -> String {
    let mut input = String::new();

    let mut num_try = TRY;

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            }
            Err(_) => {
                if num_try == 0 {
                    println!("Too many errors, exiting program");
                    std::process::exit(1);
                } else {
                    println!("Error reading input, try again");
                    num_try -= 1;
                    continue;
                }
            }
        }
    }

    input.trim().to_string()
}

pub fn input_number() -> Result<f64, String> {
    let result = input_string();

    match result.parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(result),
    }
}

pub fn get_valid_operator() -> String {
    loop {
        println!("Enter an operator (+, -, *, /): ");
        let operator = input_string();
        match operator.as_str() {
            "+" | "-" | "*" | "/" => return operator,
            EXIT_CODE => std::process::exit(0),
            _ => println!("Invalid operator, please try again."),
        }
    }
}
