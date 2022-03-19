use std::io::stdin;

pub fn main() {
    let mut input = String::new();
    
    stdin().read_line(&mut input).expect("User input expected!");
    
    let input: Vec<&str> = input.split(" ").collect();
    
    let a: i32 = parse_to_int(input[0]);
    let b: i32 = parse_to_int(input[1]);
    let c: i32 = parse_to_int(input[2]);
    
    let mut values: [i32; 3] = [a, b, c];
    
    loop {
        if values[0] < values[1] && values[1] < values[2] {
            break;
        }
        
        for i in 0..2 {
            sort_array(&mut values, i, i+1);
        }
    }
    
    println!("{}\n{}\n{}\n", values[0], values[1], values[2]);
    println!("{}\n{}\n{}", a, b, c);
}

fn parse_to_int(text: &str) -> i32 {
    text.trim().parse().unwrap()
}

fn sort_array(a: &mut [i32; 3], i1: usize, i2: usize) {
    if a[i1] > a[i2] {
        let temp = a[i2];
        a[i2] = a[i1];
        a[i1] = temp;
    }
}
