use std::io;

fn read_number() -> f32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let num: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please provide a valid number");
                continue;
            }
        };

        return num;
    }
}

fn main() {
    println!("the calculator is now running");

    println!("give first number");
    let first_num = read_number();

    println!("give second number");
    let second_num = read_number();

    println!("first_num = {} , second_num = {}", first_num, second_num);

    println!("select an operation \n 1. + \n 2. - \n 3. / \n 4. * ");

    loop {
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("failed to read operation");

        let result = operation.trim().parse();

        match result {
            Ok(1) => println!("sum = {}", first_num + second_num),
            Ok(2) => println!("difference = {}", first_num - second_num),
            Ok(3) => println!("dividend = {}", first_num / second_num),
            Ok(4) => println!("multiple =  {}", first_num * second_num),
            _ => {
                println!("select a valid number");
                continue;
            }
        };

        if result.is_ok() {
            break;
        }
    }
}
