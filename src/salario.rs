use std::io::stdin;

pub fn main() {
    let number = get_input_i16();
    let salary = calc_salary_per_hour(get_input_i16().into());

    println!("NUMBER = {}", number);
    println!("SALARY = U$ {:.2}", salary);
}

fn get_input_i16() -> i16 {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor numérico inteiro esperado!");

    match x.trim().parse() {
        Ok(res) => res,
        Err(_) => 0
    }
}

fn calc_salary_per_hour(hour: f64) -> f64 {
    get_input_f64()*hour
}

fn get_input_f64() -> f64 {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor numérico inteiro esperado!");

    match x.trim().parse() {
        Ok(res) => res,
        Err(_) => 0.0
    }
}