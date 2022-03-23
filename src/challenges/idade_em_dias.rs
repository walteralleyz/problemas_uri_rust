use std::io::stdin;

pub fn main() {
    let mut d = String::new();

    stdin().read_line(&mut d).expect("Numero inteiro esperado!");

    let d: u16 = d.trim().parse().unwrap();

    let y = d / 365;
    let m = (d % 365) / 30;
    let d = (d % 365) % 30;

    println!("{} ano(s)\n{} mes(es)\n{} dia(s)", y, m, d);
}
