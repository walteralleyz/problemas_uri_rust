use std::io::stdin;

pub fn main() {
    let mut input = String::new();
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    stdin().read_line(&mut input).expect("User input expected!");

    let input: usize = input.trim().parse().unwrap();

    println!("{}", months[input - 1]);
}
