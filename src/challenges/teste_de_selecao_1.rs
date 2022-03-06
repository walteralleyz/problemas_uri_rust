use std::io::stdin;

pub fn main() {
    let mut tup = String::new();

    stdin().read_line(&mut tup).expect("4 valores são esperados!");

    let tup: Vec<&str> = tup.split(" ").collect();

    let mut values: (i16, i16, i16, i16) = (0, 0, 0, 0);

    values.0 = tup[0].trim().parse().unwrap();
    values.1 = tup[1].trim().parse().unwrap();
    values.2 = tup[2].trim().parse().unwrap();
    values.3 = tup[3].trim().parse().unwrap();

    if values.1 > values.2
        && values.3 > values.0
        && (values.2 + values.3) > (values.0 + values.1)
        && values.2 > 0 && values.3 > 0
        && values.0 % 2 == 0 {
        println!("Valores aceitos");
    }

    else {
        println!("Valores nao aceitos");
    }
}