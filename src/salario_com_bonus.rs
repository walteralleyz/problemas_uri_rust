use std::io::stdin;

pub fn main() {
    let _nome = get_input_string();
    let salary = get_input_f64();

    println!("TOTAL = R$ {:.2}", calc_salary_by_sales(salary));
}

fn get_input_string() -> String {
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Entrada esperada!");

    x
}

fn calc_salary_by_sales(salary: f64) -> f64 {
    salary + (get_input_f64()*0.15)
}

fn get_input_f64() -> f64 {
    match get_input_string().trim().parse() {
        Ok(res) => res,
        Err(_) => 0.00
    }
}