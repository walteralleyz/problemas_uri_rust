use std::io::stdin;

pub fn main() {
    let tup: (i16, i16, i16) = get_mult_input_int();

    let maior_ab = (tup.0 + tup.1 + (tup.0 - tup.1).abs()) / 2;
    let maior_abc = (maior_ab + tup.2 + (maior_ab - tup.2).abs()) / 2;

    println!("{} eh o maior", maior_abc);
}

fn get_mult_input_int() -> (i16, i16, i16) {
    let mut tup: (i16, i16, i16) = (0, 0, 0);
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor esperado");

    let line_vec: Vec<&str> = x.split(" ").collect();

    tup.0 = line_vec[0].trim().parse().unwrap();
    tup.1 = line_vec[1].trim().parse().unwrap();
    tup.2 = line_vec[2].trim().parse().unwrap();

    tup
}
