use std::io::stdin;

pub fn main() {
    let (a, b, c) = get_input_f64();

    let inner_sqrt: f64 = (b*b) - (4.0*a*c);

    if inner_sqrt < 0.0 {
        println!("Impossivel calcular");
        return;
    }

    let sqrt = inner_sqrt.sqrt();
    let under = 2.0*a;
    let a1 = -b + sqrt;
    let a2 = -b - sqrt;

    if under == 0.0 || a1 == 0.0 || a2 == 0.0 {
        println!("Impossivel calcular");
        return;
    }

    let r1 = a1/under;
    let r2 = a2/under;

    println!("R1 = {:.5}\nR2 = {:.5}", r1, r2);
}

fn get_input_f64() -> (f64, f64, f64) {
    let mut x = String::new();
    let mut tup: (f64, f64, f64) = (0.0, 0.0, 0.0);

    stdin().read_line(&mut x).expect("Alguma entrada esperada!");

    let multiple: Vec<&str> = x.trim().split(" ").collect();

    tup.0 = multiple[0].trim().parse().unwrap();
    tup.1 = multiple[1].trim().parse().unwrap();
    tup.2 = multiple[2].trim().parse().unwrap();

    tup
}