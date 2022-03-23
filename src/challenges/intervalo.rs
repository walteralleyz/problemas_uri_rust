use std::io::stdin;

pub fn main() {
    let mut interval = String::new();
    let pairs: [[u8; 2]; 4] = [[0, 25], [25, 50], [50, 75], [75, 100]];

    stdin().read_line(&mut interval).expect("Valor esperado!");

    let interval: f64 = interval.trim().parse().unwrap();
    let mut message = String::from("Fora de intervalo");

    for pair in pairs.iter() {
        let i0 = pair[0] as f64;
        let i1 = pair[1] as f64;

        if interval >= i0 && interval <= i1 {
            message = format!(
                "Intervalo {}{},{}]",
                if i0 == 0.0 { "[" } else { "(" },
                pair[0],
                pair[1]
            );

            break;
        }
    }

    println!("{}", message);
}
