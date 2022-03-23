use std::io::stdin;

pub fn main() {
    let h = convert_string_to_i16(get_input());
    let km = convert_string_to_i16(get_input());

    let formula: f64 = (h * km) as f64 / 12.;

    println!("{:.3}", formula);
}

fn get_input() -> String {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Falta valor!");

    x
}

fn convert_string_to_i16(x: String) -> i32 {
    x.trim().parse().unwrap()
}
