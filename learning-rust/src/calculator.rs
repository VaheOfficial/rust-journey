use crate::global::input_acceptor::read_input;

pub fn start_calculator() {
    println!("Hello world - Calculator");
    loop {
        println!("Please provide x");
        let x: i32 = read_input().trim().parse().expect("Invalid input for x");
        println!("Please provide y");
        let y: i32 = read_input().trim().parse().expect("Invalid input for y");

        println!("Please provide operator (+, -, *, /, isEven)");
        let operator_input = read_input();
        let operator = operator_input.trim();

        let calculation = calculator(x, y, operator);
        println!("Your result is: {}", calculation);

        println!("Do you want to perform another calculation? (y/n)");
        let another = read_input().trim().to_string();
        if another.to_lowercase() != "y" {
            break;
        }
    }
}

fn calculator(x: i32, y: i32, operator: &str) -> String {
    match operator {
        "+" => (x + y).to_string(),
        "-" => (x - y).to_string(),
        "*" => (x * y).to_string(),
        "/" => {
            if y != 0 {
                (x / y).to_string()
            } else {
                "Division by zero".to_string()
            }
        }
        "isEven" => {
            if x % 2 == 0 {
                if y % 2 == 0 {
                    "x and y are even".to_string()
                } else {
                    "x is even".to_string()
                }
            } else {
                if y % 2 == 0 {
                    "y is even".to_string()
                } else {
                    "x and y are odd".to_string()
                }
            }
        }
        _ => "Unknown operator".to_string(),
    }
}
