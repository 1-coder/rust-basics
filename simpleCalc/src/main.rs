mod calc_helper;

fn main() {
    println!("Simple Calculator");

    loop {
        println!("Enter first number (or 'exit' to quiet the program)");

        let first_input = match calc_helper::input_number() {
            Ok(num) => num,
            Err(str) => {
                if str == calc_helper::EXIT_CODE {
                    break;
                } else {
                    println!("Invalid input, enter a valid number");
                    continue;
                }
            }
        };

        let operator = calc_helper::get_valid_operator();

        println!("Input second number");
        let second_input = match calc_helper::input_number() {
            Ok(num) => num,
            Err(str) => {
                if str == calc_helper::EXIT_CODE {
                    break;
                } else {
                    println!("Invalid input, enter a valid number");
                    continue;
                }
            }
        };

        let result = match operator.as_str() {
            "+" => first_input + second_input,
            "-" => first_input - second_input,
            "*" => first_input * second_input,
            "/" => {
                if second_input == 0.0 {
                    println!("Error: Division by zero.");
                    continue;
                } else {
                    first_input / second_input
                }
            }
            &_ => {
                println!("Invalid operator, please try again.");
                continue;
            }
        };

        println!("Result: {result}");
    }
}
