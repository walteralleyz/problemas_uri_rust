use std::io::stdin;

pub fn main() {
    let mut input_size = String::new();

    stdin()
        .read_line(&mut input_size)
        .expect("Expected to take a value");

    let input_size: u8 = input_size.trim().parse().unwrap();

    for _i in 0..input_size {
        let mut input = String::new();
        let mut count = 0;

        stdin()
            .read_line(&mut input)
            .expect("Expected to take a value");

        let number: i32 = input.clone().trim().parse().unwrap();

        for input_number in input.chars() {
            if input_number.ne(&'-')
                && input_number.ne(&'\n')
                && input_number.ne(&'\r')
                && input_number.ne(&' ')
                && input_number.ne(&'0')
            {
                let to_number: i32 = input_number.to_digit(10u32).unwrap() as i32;

                if number % to_number == 0 {
                    count += 1
                }
            }
        }

        println!("{}", count);
    }
}
