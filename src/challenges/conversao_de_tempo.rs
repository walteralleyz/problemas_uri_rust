use std::io::stdin;

pub fn main() {
    let mut s = String::new();

    stdin()
        .read_line(&mut s)
        .expect("Valor em segundos esperado!");

    let s = s.trim().parse::<u32>().unwrap();

    let hh = s / 3600;
    let mm = (s - hh * (3600)) / 60;
    let ss = s - hh * 3600 - mm * 60;

    println!("{}:{}:{}", hh, mm, ss);
}
