use std::io;

fn main() {
    loop {
        println!("Enter temperature in Celsius:");

        let mut c = String::new();

        io::stdin()
            .read_line(&mut c)
            .expect("Failed to read line");

        let c: f32 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if c.trim().to_lowercase() == "q" ||
                   c.trim().to_lowercase() == "quit"
                {
                    break;
                } else {
                    continue;
                }
            }
        };

        let f = (c * 9.0 / 5.0) + 32.0;

        println!("Temperature in Fahrenheit: {}\n", f);
    }
}
