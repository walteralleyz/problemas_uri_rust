use std::io::stdin;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let err_msg = "Digite um inteiro!";

    stdin().read_line(&mut a).expect(err_msg);
    stdin().read_line(&mut b).expect(err_msg);

    println!("SOMA = {}", get_string_as_i16(a) + get_string_as_i16(b));
}

fn get_string_as_i16(val: String) -> i16 {
    match val.trim().parse() {
        Ok(res) => res,
        Err(_) => 0
    }
}