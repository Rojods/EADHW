use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "add",
        help = "Operation to perform. Possible_values: 'add', 'sub', 'div', 'mul'. Default: 'add'."
    )]
    operation: String,
    left_number: i32,
    right_number: i32,
}
fn main() {
    let args = Args::parse();
    match args.operation.trim().to_lowercase().as_str() {
        "sub" => {
            let res = rarithm::subtract(args.left_number, args.right_number);
            match res {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        "div" => {
            let res = rarithm::divide(args.left_number, args.right_number);
            match res {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        "mul" => {
            let res = rarithm::multiply(args.left_number, args.right_number);
            match res {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        "add" => {
            let res = rarithm::add(args.left_number, args.right_number);
            match res {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        _ => {
            println!("Invalid operation: {}", args.operation);
        }
    }
}
