use std::io;

pub fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("X = {0}", get_input_int(&mut a) + get_input_int(&mut b));
}

fn get_input_int(recep: &mut String) -> i16 {
    io::stdin()
        .read_line(recep)
        .expect("Digite um numero valido!");

    match recep.trim().parse() {
        Ok(val) => val,
        Err(_) => 0,
    }
}
