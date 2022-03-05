use std::f64::consts::PI;
use std::io::stdin;

pub fn main() {
    let mut raio = String::new();
    let pi = PI;
    let d: f64 = 4.0/3.0;

    stdin().read_line(&mut raio).expect("Deve inserir um valor!");

    let r: f64 = match raio.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.0
    };

    let formula = d*pi*(r*r*r);

    println!("VOLUME = {:.3}", formula);
}

