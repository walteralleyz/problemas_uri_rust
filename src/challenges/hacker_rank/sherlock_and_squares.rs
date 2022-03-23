use std::io::stdin;

pub fn main() {
    let mut it = String::new();

    stdin().read_line(&mut it).unwrap();

    let it: u8 = it.trim().parse().unwrap();

    for _i in 0..it {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let input: Vec<&str> = input.split(" ").collect();

        let a: f64 = get_f64_sqrt(input[0]).ceil();
        let b: f64 = get_f64_sqrt(input[1]).floor();

        println!("{}", (b - a) as i32 + 1);
    }
}

fn get_f64_sqrt(text: &str) -> f64 {
    text.trim().parse::<f64>().unwrap().sqrt()
}
