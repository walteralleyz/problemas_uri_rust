use std::f64::consts::PI;
use std::io;

pub fn main() {
    let mut raio = String::new();
    let n: f64 = PI;

    io::stdin()
        .read_line(&mut raio)
        .expect("Digite um número que seja double!");

    let r: f64 = match raio.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.00,
    };

    let formula = n * (r * r);

    println!("A={:.4}", formula);
}
