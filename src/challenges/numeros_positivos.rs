use std::io::stdin;

pub fn main() {
    let mut positive_count = 0;
    
    for _i in 0..6 {
        let temp = get_input_f64();
        
        if temp > 0. {
            positive_count += 1;
        }
    }
    
    println!("{} valores positivos", positive_count);
}

fn get_input_f64() -> f64 {
    let mut input = String::new();
    
    stdin().read_line(&mut input).expect("User input expected!");
    
    input.trim().parse().unwrap()
}
