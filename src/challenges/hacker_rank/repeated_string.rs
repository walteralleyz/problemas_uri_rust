use std::io::stdin;

pub fn main() {
    let s = get_input_string();
    let n: usize = get_input_string().trim().parse().unwrap();

    let d: usize = n / s.trim().len();
    let r: usize = n % s.trim().len();

    let d = s.matches("a").count() * d;
    let r = *&s[..r].matches("a").count();

    println!("{}", d + r);
}

fn get_input_string() -> String {
    let mut temp = String::new();

    stdin().read_line(&mut temp).unwrap();

    temp
}
