use std::io::stdin;

pub fn main() {
    println!("PROD = {}", get_input_i16() * get_input_i16());
}

fn get_input_i16() -> i16 {
    let mut x = String::new();

    stdin()
        .read_line(&mut x)
        .expect("Valor numérico inteiro esperado!");

    match x.trim().parse() {
        Ok(res) => res,
        Err(_) => 0,
    }
}
