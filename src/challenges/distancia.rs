use std::io::stdin;

pub fn main() {
    let km = convert_to_i16(get_input());

    println!("{} minutos", km * 2);
}

fn get_input() -> String {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor não pode ser vazio");

    x
}

fn convert_to_i16(x: String) -> i16 {
    x.trim().parse().unwrap()
}
