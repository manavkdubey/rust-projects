mod calculator;

fn main() -> Result<(), calculator::Error> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = calculator::Calculator::parse(input);
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr = calculator::Calculator::expression(tokens?);
                if let Some(v) = calculator::Calculator::evaluate(expr) {
                    println!("{}", v);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
