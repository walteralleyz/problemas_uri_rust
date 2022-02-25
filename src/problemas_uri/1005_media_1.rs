use std::io::stdin;

fn main() {
    let formula: f64 = (get_input_f64()*3.5) + (get_input_f64()*7.5);
    println!("MEDIA = {:.5}", formula / 11.0);
}

fn get_input_f64() -> f64 {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor numérico de dupla precisão esperado!");

    match x.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.00
    }
}