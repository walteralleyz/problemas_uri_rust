use std::io::stdin;

pub fn main() {
    let p1 = get_x_y();
    let p2 = get_x_y();

    let r1 = (p2.0 - p1.0) * (p2.0 - p1.0);
    let r2 = (p2.1 - p1.1) * (p2.1 - p1.1);

    let formula = (r1 + r2).sqrt();

    println!("{:.4}", formula);
}

fn get_x_y() -> (f64, f64) {
    let mut tup: (f64, f64) = (0., 0.);
    let mut x = String::new();

    stdin().read_line(&mut x).expect("Valor esperado!");

    tup.0 = split_string_and_convert_to_f64(&x, 0);
    tup.1 = split_string_and_convert_to_f64(&x, 1);

    tup
}

fn split_string_and_convert_to_f64(x: &String, pos: usize) -> f64 {
    x.split(" ").collect::<Vec<&str>>()[pos]
        .trim()
        .parse()
        .unwrap()
}
