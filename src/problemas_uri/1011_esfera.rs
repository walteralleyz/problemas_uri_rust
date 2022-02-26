use std::io::stdin;

fn main() {
    let mut raio = String::new();
    let pi = 3.14159;
    let d: f64 = 4.0/3.0;

    stdin().read_line(&mut raio).expect("Deve inserir um valor!");

    let r: f64 = match raio.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.0
    };

    let formula = d*pi*(r*r*r);

    println!("VOLUME = {:.3}", formula);
}

