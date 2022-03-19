use std::io::stdin;
use std::env;
use std::collections::hash_map::HashMap;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let challenges: HashMap<&str, fn()> = HashMap::from([
        ("find_digits", get_as_fn(FindDigits::main)),
        ("sherlock_and_squares", get_as_fn(SherlockAndSquares::main)),
        ("library_fine", get_as_fn(LibraryFine::main))
    ]);

    let option = args.get(2).expect("Challenge title expected!");

    challenges.get(&option.as_str()).unwrap()();
}

fn get_as_fn(f: fn() -> ()) -> fn() {
    f as fn()
}

struct LibraryFine;

impl LibraryFine {
    fn main() {
        let mut return_date = String::new();
        let mut due_date = String::new();

        stdin().read_line(&mut return_date).unwrap();
        stdin().read_line(&mut due_date).unwrap();

        let return_date: Vec<&str> = return_date.split(" ").collect();
        let due_date: Vec<&str> = due_date.split(" ").collect();

        let (d1, m1, y1) = (LibraryFine::parse_to_u16(return_date[0]), LibraryFine::parse_to_u16(return_date[1]), LibraryFine::parse_to_u16(return_date[2]));
        let (d2, m2, y2) = (LibraryFine::parse_to_u16(due_date[0]), LibraryFine::parse_to_u16(due_date[1]), LibraryFine::parse_to_u16(due_date[2]));

        println!("{}", if y1 < y2 || (y1 == y2 && ((m1 == m2 && d1 <= d2) || m1 < m2)) { String::from("0") }
            else if m1 == m2 && y1 == y2 { format!("{}", 15 * (d1-d2)) }
            else if y1 > y2 { String::from("10000") }
            else { 
                let months1 = y1*12 + m1;
                let months2 = y2*12 + m2;
                let result: u16 = 500 * (months1-months2);

                result.to_string()
            }
        );
    }

    fn parse_to_u16(text: &str) -> u16 {
        text.trim().parse::<u16>().unwrap()
    }
}

struct SherlockAndSquares;

impl SherlockAndSquares {
    fn main() {
        let mut it = String::new();

        stdin().read_line(&mut it).unwrap();

        let it: u8 = it.trim().parse().unwrap();

        for _i in 0..it {
            let mut input = String::new();

            stdin().read_line(&mut input).unwrap();

            let input: Vec<&str> = input.split(" ").collect();

            let a: f64 = SherlockAndSquares::get_f64_sqrt(input[0]).ceil();
            let b: f64 = SherlockAndSquares::get_f64_sqrt(input[1]).floor();

            println!("{}", (b-a) as i32 + 1);
        }
    }

    fn get_f64_sqrt(text: &str) -> f64 {
        text.trim().parse::<f64>().unwrap().sqrt()
    }
}

struct FindDigits;

impl FindDigits {
    fn main() {
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
}
