use std::{io::Write, str::FromStr};

fn parse<T, S>(prompt: S, error: S) -> T
where T: FromStr,
      S: AsRef<str>
{
    let mut input: String;
    loop
    {
        print!("{}", prompt.as_ref());
        std::io::stdout().flush().expect("Failed to flush");
        input = String::from("");

        std::io::stdin().read_line(&mut input).expect("Invalid input");
        input = String::from(input.trim());

        match input.parse::<T>()
        {
            Ok(val) => return val,
            Err(_) =>
            {
                println!("{}", error.as_ref());
                input.clear();
            }
        }
    }
}

fn prompt<S>(prompt: S) -> String
where S: AsRef<str>
{
    let mut input: String;
    print!("{}", prompt.as_ref());
    std::io::stdout().flush().expect("Failed to flush");
    input = String::from("");

    std::io::stdin().read_line(&mut input).expect("Invalid input");
    input = String::from(input.trim());

    return input;
}

fn main() {
    let name: String = prompt("Insert your name: ");

    let age: i32 = parse("How old are you? ", "type in an integer please");

    println!("{name} is {age} years old!");
}