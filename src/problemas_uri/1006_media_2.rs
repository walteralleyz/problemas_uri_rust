use std::io::stdin;

fn main() {
    let formula: f64 = calc_x_by_weight(2.0) + calc_x_by_weight(3.0) + calc_x_by_weight(5.0);
    println!("MEDIA = {:.1}", formula / 10.0);
}

fn calc_x_by_weight(weight: f64) -> f64 {
    get_input_f64() * weight
}

fn get_input_f64() -> f64 {
    let mut x = String::new();
    
    stdin().read_line(&mut x).expect("Valor numérico dupla precisão esperado!");

    match x.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.00
    }
}