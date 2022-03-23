use std::io::stdin;
use std::str::FromStr;

pub fn main() {
    let km = convert_string_to_type::<f64>(get_input_string());
    let gas = convert_string_to_type::<f64>(get_input_string());

    println!("{:.3} km/l", km / gas);
}

fn get_input_string() -> String {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor esperado");

    x
}

fn convert_string_to_type<T: FromStr>(x: String) -> T {
    match x.trim().parse::<T>() {
        Ok(val) => val,
        Err(_) => panic!("Erro ao converter"),
    }
}
