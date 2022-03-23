use std::io::stdin;

pub fn main() {
    let mut count = 0;
    let mut sum: f64 = 0.0;

    loop {
        if count == 2 {
            break;
        }

        let line = get_input_string();
        let values: Vec<&str> = line.split(" ").collect();

        let qntd: f64 = cast_string_to_f64(values[1].to_string());
        let price: f64 = cast_string_to_f64(values[2].to_string());

        sum += price * qntd;

        count += 1;
    }

    println!("VALOR A PAGAR: R$ {:.2}", sum);
}

fn get_input_string() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Valor inválido");

    input
}

fn cast_string_to_f64(value: String) -> f64 {
    match value.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.0,
    }
}
